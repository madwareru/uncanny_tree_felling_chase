use{
    macroquad::prelude::*
};
use core_subsystems::forest::Forest;
use core_subsystems::rendering::SceneCompositor;
use core_subsystems::tilemap::Tilemap;
use core_subsystems::types::GlobalContext;
use crate::components::{
    MenuBackgroundTag,
    UiRect,
    SignalButton,
    PlayGameSignal,
    ExitGameSignal,
    MenuScreenElement,
    SignalTag,
    ChoosePlayerFractionSignal,
    GoToMainMenuSignal,
    Glyph
};
use crate::core_subsystems::types::{MenuScreen, GameState, BattleState, Fraction};
use std::cell::{RefCell};
use std::ops::Deref;
use std::collections::VecDeque;

mod game_assets;
mod core_subsystems;
mod components;
mod systems;

fn window_conf() -> Conf {
    Conf {
        window_title: "Uncanny treefelling chase".to_owned(),
        fullscreen: false,
        window_width: 1280,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let game_assets::GameAssets{
        atlas_definition,
        atlas_texture,
        ui_atlas_texture,
    } = game_assets::GameAssets::load();
    clear_background(Color::new(0.12, 0.1, 0.15, 1.00));

    next_frame().await;
    let mut tilemap = Tilemap::new(atlas_definition.clone(), 60, 40);
    let mut forest = Forest::create(&tilemap);
    tilemap.generate_new_map();
    forest.plant_trees(&tilemap);

    let scene_compositor = SceneCompositor::new();

    let mut global_context = GlobalContext {
        atlas_texture,
        draw_scale: 1.0 / 8.0,
        atlas_definition: atlas_definition.clone(),
        ui_atlas_texture,
        tilemap: RefCell::new(tilemap),
        forest: RefCell::new(forest),
        scene_compositor: RefCell::new(scene_compositor),
        world: RefCell::new(hecs::World::new()),
        game_state: RefCell::new(GameState::MainMenu),
        signal_command_buffer: RefCell::new(VecDeque::new()),
        entity_purgatory: RefCell::new(VecDeque::new())
    };

    create_ui_screens(&mut global_context);

    macro_rules! exec_system {
        ($($namespace: ident)::*) => {
            systems::$($namespace::)*system(&global_context);
            global_context.flush_command_queues();
        }
    }

    'main_loop: loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(Color::new(0.12, 0.1, 0.15, 1.00));
        exec_system! [cleanup_signals];

        //exec_system! [rendering::tilemap];
        //exec_system! [rendering::forest];

        exec_system! [gameplay::button_system];

        for (_, _) in global_context.world.borrow()
            .query::<(&SignalTag, &ExitGameSignal)>()
            .iter()
        {
            break 'main_loop;
        }

        let next_state = 'state_search: loop {
            match global_context.game_state.borrow().deref() {
                GameState::MainMenu => {
                    for (_, _) in global_context.world.borrow()
                        .query::<(&SignalTag, &PlayGameSignal)>()
                        .iter()
                    {
                        break 'state_search GameState::FractionChoice;
                    }
                    break 'state_search GameState::MainMenu;
                }
                GameState::FractionChoice => {
                    for (_, _) in global_context.world.borrow()
                        .query::<(&SignalTag, &GoToMainMenuSignal)>()
                        .iter()
                    {
                        break 'state_search GameState::MainMenu;
                    }

                    for (_, (_, signal)) in global_context.world.borrow()
                        .query::<(&SignalTag, &ChoosePlayerFractionSignal)>()
                        .iter()
                    {
                        break 'state_search GameState::Battle {
                            fraction: signal.fraction,
                            internal_state: BattleState::MapGeneration
                        };
                    }

                    break 'state_search GameState::FractionChoice;
                }
                GameState::Battle { fraction, internal_state } => {
                    match internal_state {
                        BattleState::MapGeneration => {
                            global_context.tilemap.borrow_mut().generate_new_map();
                            global_context.forest.borrow_mut().plant_trees(
                                &global_context.tilemap.borrow()
                            );
                            break 'state_search GameState::Battle {
                                fraction: *fraction,
                                internal_state: BattleState::EnemyLanding
                            };
                        }
                        BattleState::EnemyLanding => {
                            // todo: Land enemies
                            break 'state_search GameState::Battle {
                                fraction: *fraction,
                                internal_state: BattleState::PlayerLanding {
                                    current_minion_is_big: false
                                }
                            };
                        }
                        BattleState::PlayerLanding { .. } => {}
                        BattleState::BattlePause => {}
                        BattleState::Defeat => {}
                        BattleState::Victory => {}
                        BattleState::Simulation { .. } => {}
                    }

                    if *internal_state != BattleState::MapGeneration {
                        exec_system! [rendering::tilemap];
                        exec_system! [rendering::forest];
                    }

                    break 'state_search GameState::Battle {
                        fraction: *fraction,
                        internal_state: *internal_state
                    };
                }
            }
        };

        *global_context.game_state.borrow_mut() = next_state;

        exec_system! [rendering::ui_overlay_main];
        exec_system! [rendering::menu_screens];
        exec_system! [scene_composition];

        next_frame().await;
    }
}

fn create_ui_screens(ctx: &GlobalContext) {
    create_main_menu_screen(ctx);
    create_choose_fraction_screen(ctx);
}

fn create_main_menu_screen(ctx: &GlobalContext) {
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::MainMenu },
        MenuBackgroundTag,
        UiRect {
            top_left: (20, 13),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 21, 24)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::MainMenu },
        SignalButton {
            signal_to_send: PlayGameSignal,
            glyph_sub_rect: ctx.atlas_definition.play_button_subrect
        },
        UiRect {
            top_left: (22, 15),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 23, 18)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::MainMenu },
        SignalButton {
            signal_to_send: ExitGameSignal,
            glyph_sub_rect: ctx.atlas_definition.exit_button_subrect
        },
        UiRect {
            top_left: (22, 19),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 23, 22)
        }
    ));
}

fn create_choose_fraction_screen(ctx: &GlobalContext) {
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        Glyph { glyph_sub_rect: ctx.atlas_definition.choose_your_side_title_subrect},
        UiRect {
            top_left: (12, 11),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 13, 12)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        MenuBackgroundTag,
        UiRect {
            top_left: (12, 13),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 13, 24)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        SignalButton {
            signal_to_send: ChoosePlayerFractionSignal{ fraction: Fraction::Red },
            glyph_sub_rect: ctx.atlas_definition.red_button_subrect
        },
        UiRect {
            top_left: (22 - 8, 15),
            bottom_right: (37 - 8, 18)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        SignalButton {
            signal_to_send: ChoosePlayerFractionSignal{ fraction: Fraction::Blue },
            glyph_sub_rect: ctx.atlas_definition.blue_button_subrect
        },
        UiRect {
            top_left: (22 + 8, 15),
            bottom_right: (37 + 8, 18)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        SignalButton {
            signal_to_send: GoToMainMenuSignal,
            glyph_sub_rect: ctx.atlas_definition.back_button_subrect
        },
        UiRect {
            top_left: (22, 19),
            bottom_right: (37, 22)
        }
    ));
}
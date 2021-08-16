use{
    macroquad::prelude::*
};
use core_subsystems::forest::Forest;
use core_subsystems::rendering::SceneCompositor;
use core_subsystems::tilemap::Tilemap;
use core_subsystems::types::GlobalContext;
use crate::core_subsystems::types::{ GameState, BattleState };
use std::cell::{RefCell};
use std::ops::Deref;
use std::collections::VecDeque;
use crate::core_subsystems::ui_layouts::{
    create_main_menu_screen,
    create_choose_fraction_screen,
    create_player_landing_screen
};
use crate::core_subsystems::signal_utils::check_signal;
use crate::components::{
    ExitGameSignal,
    PlayGameSignal,
    GoToMainMenuSignal,
    ChoosePlayerFractionSignal
};

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
        passability_atlas_width,
        passability_atlas_height,
        passability_atlas,
    } = game_assets::GameAssets::load();
    clear_background(Color::new(0.12, 0.1, 0.15, 1.00));

    next_frame().await;
    let mut tilemap = Tilemap::new(atlas_definition.clone(), 60, 40);
    let mut forest = Forest::create(&tilemap);

    let scene_compositor = SceneCompositor::new();

    let mut global_context = GlobalContext {
        atlas_texture,
        passability_atlas_width,
        passability_atlas_height,
        passability_atlas,
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
        exec_system! [gameplay::button_system];

        if check_signal::<ExitGameSignal>(&global_context).is_some() {
            break 'main_loop;
        }

        let next_state = 'state_search: loop {
            match global_context.game_state.borrow().deref() {
                GameState::MainMenu => {
                    if check_signal::<PlayGameSignal>(&global_context).is_some() {
                        break 'state_search GameState::FractionChoice;
                    }
                    break 'state_search GameState::MainMenu;
                }
                GameState::FractionChoice => {
                    if check_signal::<GoToMainMenuSignal>(&global_context).is_some() {
                        break 'state_search GameState::MainMenu;
                    }
                    if let Some(signal) = check_signal::<ChoosePlayerFractionSignal>(&global_context) {
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
                            exec_system! [gameplay::update_landing_ui];
                            // todo: Land enemies
                            break 'state_search GameState::Battle {
                                fraction: *fraction,
                                internal_state: BattleState::PlayerLanding {
                                    budget: 1234567890,
                                    current_minion_is_big: false
                                }
                            };
                        }
                        BattleState::PlayerLanding { .. } => {
                            exec_system! [gameplay::update_landing_ui];


                            exec_system! [rendering::player_landing_helper_grid];
                        }
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
    create_player_landing_screen(ctx);
}
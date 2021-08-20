use {
    macroquad::prelude::*
};
use core_subsystems::forest::Forest;
use core_subsystems::rendering::SceneCompositor;
use core_subsystems::tilemap::Tilemap;
use core_subsystems::types::GlobalContext;
use crate::core_subsystems::types::{GameState, BattleState};
use std::cell::{RefCell};
use std::ops::Deref;
use std::collections::VecDeque;
use crate::core_subsystems::ui_layouts::{
    create_main_menu_screen,
    create_choose_fraction_screen,
    create_player_landing_screen,
};
use crate::core_subsystems::signal_utils::check_signal;
use crate::components::{ExitGameSignal, PlayGameSignal, GoToMainMenuSignal, ChoosePlayerFractionSignal, ChooseUnitTypeDuringLanding, ReplayGameSignal};
use macro_tiler::atlas::rect_handle::{Having, DrawSizeOverride, DrawPivot, DrawRotation};
use crate::core_subsystems::atlas_serialization::UiTile;
use crate::core_subsystems::rendering::RenderLayer;

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
    let game_assets::GameAssets {
        main_atlas_definition,
        ui_atlas_definition,
        main_atlas,
        ui_atlas,
        units_config, atlas_definition,
        passability_atlas_width,
        passability_atlas_height,
        passability_atlas,
    } = game_assets::GameAssets::load();
    const MAP_SIZE: [usize; 2] = [70, 35];

    clear_background(Color::new(0.12, 0.1, 0.15, 1.00));

    next_frame().await;
    let tilemap = Tilemap::new(atlas_definition.clone(), MAP_SIZE[0], MAP_SIZE[1]);
    let forest = Forest::create(&tilemap);
    let passability_map_stride = atlas_definition.tile_width * tilemap.w;
    let passability_map = vec![0x00u8; passability_map_stride * tilemap.h * atlas_definition.tile_height];

    let scene_compositor = SceneCompositor::new();

    let mut global_context = GlobalContext {
        main_atlas_definition,
        ui_atlas_definition,
        main_atlas,
        ui_atlas,
        passability_atlas_width,
        passability_atlas_height,
        passability_atlas,
        passability_map_stride,
        draw_scale: 1.0 / 8.0,

        atlas_scheme: atlas_definition.clone(),
        units_config: units_config.clone(),

        passability_map: RefCell::new(passability_map),
        tilemap: RefCell::new(tilemap),
        forest: RefCell::new(forest),
        scene_compositor: RefCell::new(scene_compositor),
        world: RefCell::new(hecs::World::new()),
        game_state: RefCell::new(GameState::MainMenu),
        signal_command_buffer: RefCell::new(VecDeque::new()),
        entity_purgatory: RefCell::new(VecDeque::new()),
    };

    create_ui_screens(&mut global_context);

    macro_rules! exec_system {
        ($($namespace: ident)::*) => {
            systems::$($namespace::)*system(&global_context);
            global_context.flush_command_queues();
        }
    }

    'main_loop: loop {
        clear_background(Color::new(0.12, 0.1, 0.15, 1.00));

        exec_system![cleanup_signals];
        exec_system![gameplay::button_system];
        exec_system![gameplay::glyph_fade];

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
                            internal_state: BattleState::MapGeneration,
                        };
                    }
                    break 'state_search GameState::FractionChoice;
                }
                GameState::Battle { fraction, internal_state } => {
                    match internal_state {
                        BattleState::MapGeneration => {
                            const MINIMUM_TREE_AMOUNT: usize = 600;
                            if !global_context.tilemap.borrow().is_busy() {
                                global_context.tilemap.borrow_mut().generate_new_map();
                            } else if global_context.tilemap.borrow_mut().poll() {
                                global_context.forest.borrow_mut().plant_trees(
                                    &global_context.tilemap.borrow()
                                );
                                if global_context.forest.borrow().total_planted() >= MINIMUM_TREE_AMOUNT {
                                    break 'state_search GameState::Battle {
                                        fraction: *fraction,
                                        internal_state: BattleState::PreparePlayerLanding,
                                    };
                                }
                            }

                            let draw_command = macro_tiler::atlas::draw_command::draw_command_builder()
                                .having(DrawSizeOverride::ScaledUniform(2.0))
                                .having(DrawPivot::Relative([0.5, 0.5].into()))
                                .build(
                                    global_context.ui_atlas.acquire_handle(&UiTile::GenerateInProgress).unwrap(),
                                    (global_context.tilemap.borrow().w / 2 * global_context.atlas_scheme.tile_width) as f32,
                                    (global_context.tilemap.borrow().h / 2 * global_context.atlas_scheme.tile_height) as f32,
                                );
                            global_context.scene_compositor.borrow_mut().enqueue(RenderLayer::Custom(20), draw_command);
                        }
                        BattleState::PreparePlayerLanding => {
                            exec_system![gameplay::update_landing_ui];
                            exec_system![gameplay::update_pasability_map];
                            break 'state_search GameState::Battle {
                                fraction: *fraction,
                                internal_state: BattleState::PlayerLanding {
                                    budget: 100000,
                                    current_minion_is_big: false,
                                },
                            };
                        }
                        BattleState::PlayerLanding { budget, .. } => {
                            exec_system![gameplay::update_landing_ui];
                            if check_signal::<GoToMainMenuSignal>(&global_context).is_some() {
                                break 'state_search GameState::MainMenu;
                            }
                            if check_signal::<ReplayGameSignal>(&global_context).is_some() {
                                break 'state_search GameState::Battle {
                                    fraction: *fraction,
                                    internal_state: BattleState::MapGeneration,
                                };
                            }

                            if let Some(signal) = check_signal::<ChooseUnitTypeDuringLanding>(&global_context) {
                                break 'state_search GameState::Battle {
                                    fraction: *fraction,
                                    internal_state: BattleState::PlayerLanding {
                                        budget: *budget,
                                        current_minion_is_big: signal.new_minion_is_big,
                                    },
                                };
                            }
                        }
                        BattleState::EnemyLanding => {
                            // todo: Land enemies
                        }
                        BattleState::BattlePause => {}
                        BattleState::Defeat => {}
                        BattleState::Victory => {}
                        BattleState::Simulation { .. } => {}
                    }

                    break 'state_search GameState::Battle {
                        fraction: *fraction,
                        internal_state: *internal_state,
                    };
                }
            }
        };

        match next_state {
            GameState::Battle { internal_state: BattleState::MapGeneration, .. } => {}
            GameState::Battle { internal_state, .. } => {
                exec_system![rendering::tilemap];
                exec_system![rendering::forest];
                if let BattleState::PlayerLanding { .. } = internal_state {
                    exec_system![rendering::player_landing_helper_grid];
                }
            }
            _ => {}
        }

        *global_context.game_state.borrow_mut() = next_state;

        exec_system![rendering::ui_overlay_main];
        exec_system![rendering::menu_screens];
        exec_system![scene_composition];

        next_frame().await;
    }
}

fn create_ui_screens(ctx: &GlobalContext) {
    create_main_menu_screen(ctx);
    create_choose_fraction_screen(ctx);
    create_player_landing_screen(ctx);
}
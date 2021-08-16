use{
    macroquad::prelude::*
};
use core_subsystems::forest::Forest;
use core_subsystems::rendering::SceneCompositor;
use core_subsystems::tilemap::Tilemap;
use core_subsystems::types::GlobalContext;
use crate::components::{MenuBackgroundTag, UiRect, SignalButton, PlayGameSignal, ExitGameSignal, MenuScreenElement, SignalTag};
use hecs::{World, Entity};
use std::borrow::Borrow;
use std::sync::Arc;
use crate::core_subsystems::atlas_serialization::AtlasDefinition;
use crate::core_subsystems::types::{MenuScreen, GameState};
use std::cell::{RefCell, Ref};
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
        tilemap,
        forest,
        atlas_texture,
        draw_scale: 1.0 / 8.0,
        atlas_definition: atlas_definition.clone(),
        ui_atlas_texture,
        scene_compositor: RefCell::new(scene_compositor),
        world: RefCell::new(hecs::World::new()),
        game_state: RefCell::new(GameState::MainMenu),
        signal_command_buffer: RefCell::new(VecDeque::new())
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
        //exec_system! [rendering::tilemap];
        //exec_system! [rendering::forest];

        for (_, _) in global_context.world.borrow()
            .query::<(&SignalTag, &ExitGameSignal)>()
            .iter()
        {
            break 'main_loop;
        }

        match global_context.game_state.borrow().deref() {
            GameState::MainMenu => {

            }
            GameState::FractionChoice => {

            }
            GameState::Battle { .. } => {

            }
        }

        exec_system! [rendering::ui_overlay_main];
        exec_system! [rendering::menu_screens];
        exec_system! [gameplay::button_system];

        exec_system! [scene_composition];
        next_frame().await;
    }
}

fn create_ui_screens(ctx: &GlobalContext) {
    create_main_menu_screen(ctx);
}

fn create_main_menu_screen(ctx: &GlobalContext) {
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::MainMenu },
        MenuBackgroundTag,
        UiRect {
            top_left: (20, 13),
            bottom_right: (ctx.tilemap.w as i32 - 21, 24)
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
            bottom_right: (ctx.tilemap.w as i32 - 23, 18)
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
            bottom_right: (ctx.tilemap.w as i32 - 23, 22)
        }
    ));
}
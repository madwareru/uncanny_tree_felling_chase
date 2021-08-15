use{
    macroquad::prelude::*
};
use core_subsystems::forest::Forest;
use core_subsystems::rendering::SceneCompositor;
use core_subsystems::tilemap::Tilemap;
use core_subsystems::types::GlobalStorage;

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

    let mut world = hecs::World::new();
    world.spawn((GlobalStorage{
        tilemap,
        forest,
        scene_compositor,
        atlas_texture,
        draw_scale: 1.0 / 8.0,
        atlas_definition: atlas_definition.clone(),
        ui_atlas_texture
    },));
    macro_rules! exec_system {
        ($($namespace: ident)::*) => {
            systems::$($namespace::)*system(&mut world);
        }
    }

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(Color::new(0.12, 0.1, 0.15, 1.00));
        //exec_system! [rendering::tilemap];
        //exec_system! [rendering::forest];

        exec_system! [debug_picking];

        exec_system! [rendering::ui_overlay_main];
        exec_system! [rendering::ui_main_menu];

        exec_system! [scene_composition];
        next_frame().await;
    }
}
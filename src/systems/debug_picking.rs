use macroquad::prelude::*;
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra};
use crate::core_subsystems::peek_utils::peek_tile;
use crate::core_subsystems::types::GlobalStorage;

pub fn system(world: &mut hecs::World) {
    for (_, data) in world.query_mut::<(&mut crate::core_subsystems::types::GlobalStorage,)>() {
        let global_storage: &mut GlobalStorage = data.0;
        let (tile_x, tile_y) = peek_tile(global_storage);
        global_storage.scene_compositor.enqueue(
            DrawCommand {
                tex: global_storage.ui_atlas_texture,
                subrect: global_storage.atlas_definition.ui_background_box_3x3_4,
                x: tile_x as f32 * global_storage.atlas_definition.tile_width as f32,
                y: tile_y as f32 * global_storage.atlas_definition.tile_height as f32,
                scale: 1.0,
                drawing_extra: DrawCommandExtra::Draw,
                sorting_layer: 10
            }
        );
    }
}
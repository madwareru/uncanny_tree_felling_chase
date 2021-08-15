use crate::core_subsystems::types::GlobalStorage;
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra};

pub fn system(world: &mut hecs::World) {
    for (_, data) in world.query_mut::<(&mut GlobalStorage,)>() {
        let global_storage: &mut GlobalStorage = data.0;
        for j in 0..global_storage.tilemap.h {
            for i in 0..global_storage.tilemap.w {
                let idx = j * global_storage.tilemap.w + i;
                let idx = global_storage.tilemap.map_data[idx];
                let draw_command = DrawCommand {
                    tex: global_storage.atlas_texture,
                    subrect: global_storage.tilemap.tiles[idx],
                    x: (i * global_storage.atlas_definition.tile_width) as f32,
                    y: (j * global_storage.atlas_definition.tile_height) as f32,
                    scale: 1.0,
                    drawing_extra: DrawCommandExtra::Draw,
                    sorting_layer: -1
                };
                global_storage.scene_compositor.enqueue(draw_command);
            }
        }
    }
}
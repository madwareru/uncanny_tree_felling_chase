use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra, SceneCompositor};

pub fn system(ctx: &GlobalContext) {
    for j in 0..ctx.tilemap.h {
        for i in 0..ctx.tilemap.w {
            let idx = j * ctx.tilemap.w + i;
            let idx = ctx.tilemap.map_data[idx];
            let draw_command = DrawCommand {
                tex: ctx.atlas_texture,
                subrect: ctx.tilemap.tiles[idx],
                x: (i * ctx.atlas_definition.tile_width) as f32,
                y: (j * ctx.atlas_definition.tile_height) as f32,
                scale: 1.0,
                drawing_extra: DrawCommandExtra::Draw,
                sorting_layer: -1
            };
            ctx.scene_compositor.borrow_mut().enqueue(draw_command);
        }
    }
}
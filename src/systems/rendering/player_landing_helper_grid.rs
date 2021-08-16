use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::atlas_serialization::{TerrainType, TreeType};
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra};
use crate::core_subsystems::peek_utils::peek_tile;

pub fn system(ctx: &GlobalContext) {
    for j in 0..ctx.tilemap.borrow().h {
        for i in 0..ctx.tilemap.borrow().w {
            let idx = j * ctx.tilemap.borrow().w + i;
            let tile_idx = ctx.tilemap.borrow().map_data[idx];
            let tile_sides = ctx.tilemap.borrow().tile_sides[tile_idx];
            let is_blocked = tile_sides.north_east == TerrainType::Water ||
                tile_sides.north_west == TerrainType::Water ||
                tile_sides.south_east == TerrainType::Water ||
                tile_sides.south_west == TerrainType::Water ||
                ctx.forest.borrow().corner_tree_data[idx] != TreeType::None ||
                ctx.forest.borrow().cell_tree_data[idx] != TreeType::None;

            if is_blocked {
                for jj in 0..2 {
                    for ii in 0..2 {
                        let draw_command = DrawCommand {
                            tex: ctx.ui_atlas_texture,
                            subrect: ctx.atlas_definition.blocked_cell_subrect,
                            x: (i as f32 + 0.5 * ii as f32) * ctx.atlas_definition.tile_width as f32,
                            y: (j as f32 + 0.5 * jj as f32) * ctx.atlas_definition.tile_height as f32,
                            scale: 0.5,
                            drawing_extra: DrawCommandExtra::Draw,
                            sorting_layer: 1
                        };
                        ctx.scene_compositor.borrow_mut().enqueue(draw_command);
                    }
                }
            } else {
                let (peek_x, peek_y) = peek_tile(ctx);
                let draw_command = DrawCommand {
                    tex: ctx.ui_atlas_texture,
                    subrect: if peek_x == (i as i32) && peek_y == (j as i32) {
                        ctx.atlas_definition.selected_cell_subrect
                    } else {
                        ctx.atlas_definition.ok_cell_subrect
                    },
                    x: (i * ctx.atlas_definition.tile_width) as f32,
                    y: (j * ctx.atlas_definition.tile_width) as f32,
                    scale: 0.5,
                    drawing_extra: DrawCommandExtra::Draw,
                    sorting_layer: 1
                };
                ctx.scene_compositor.borrow_mut().enqueue(draw_command);
            }
        }
    }
}
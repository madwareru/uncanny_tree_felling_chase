use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::atlas_serialization::{TreeType};
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra, Pivot};
use crate::core_subsystems::peek_utils::{peek_tile_ext};
use crate::core_subsystems::passability_utils::is_half_tile_passable;

pub fn system(ctx: &GlobalContext) {
    let half_tile_w = ctx.atlas_definition.tile_width / 2;
    let half_tile_h = ctx.atlas_definition.tile_height / 2;

    for j in 0..ctx.tilemap.borrow().h * 2 {
        for i in 0..ctx.tilemap.borrow().w * 2 {
            let is_passable = is_half_tile_passable(ctx, i, j);
            let is_blocked = !is_passable || {
                let idx = (j / 2) * ctx.tilemap.borrow().w + i / 2;
                ctx.forest.borrow().corner_tree_data[idx] != TreeType::None ||
                    ctx.forest.borrow().cell_tree_data[idx] != TreeType::None
            };

            let (peek_x, peek_y) = peek_tile_ext(ctx, 0.5);
            let draw_command = DrawCommand {
                tex: ctx.ui_atlas_texture,
                subrect:
                    if is_blocked {
                        ctx.atlas_definition.blocked_cell_subrect
                    } else {
                        if peek_x == (i as i32) && peek_y == (j as i32) {
                            ctx.atlas_definition.selected_cell_subrect
                        } else {
                            ctx.atlas_definition.ok_cell_subrect
                        }
                    }
                ,
                x: (i as f32 + 0.5) * half_tile_w as f32,
                y: (j as f32 + 0.5) * half_tile_h as f32,
                scale:
                if peek_x == (i as i32) && peek_y == (j as i32) {
                    0.5
                } else {
                    match (peek_x - i as i32).abs().max((peek_y - j as i32).abs()) {
                        1 => 0.42,
                        2 => 0.36,
                        _ => 0.25
                    }
                },
                drawing_extra: DrawCommandExtra::DrawWithPivot {
                    pivot: Pivot::Relative {
                        rel_x: 0.5,
                        rel_y: 0.5
                    }
                },
                sorting_layer: 1
            };
            ctx.scene_compositor.borrow_mut().enqueue(draw_command);
        }
    }
}
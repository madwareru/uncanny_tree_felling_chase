use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra, Pivot, SceneCompositor};

pub fn system(ctx: &GlobalContext) {
    let mut y = 0.0;
    let dy = ctx.atlas_definition.tile_height as f32;
    let dx = ctx.atlas_definition.tile_width as f32;

    for idx in (0..ctx.tilemap.w*ctx.tilemap.h)
        .step_by(ctx.tilemap.w)
    {
        for i in 0..ctx.tilemap.w {
            let corner_tree = ctx.forest.corner_tree_data[idx + i];
            if let Some(&subrect) = ctx.atlas_definition.tree_sub_rects.get(&corner_tree) {
                let x = i as f32 * dx;
                let draw_command = DrawCommand {
                    tex: ctx.atlas_texture,
                    subrect,
                    x, y,
                    scale: 1.0,
                    drawing_extra: DrawCommandExtra::DrawWithPivot {
                        pivot: Pivot::Relative {
                            rel_x: 0.5,
                            rel_y: 0.95
                        }
                    },
                    sorting_layer: 0
                };
                ctx.scene_compositor.borrow_mut().enqueue(draw_command);
            }

            let cell_tree = ctx.forest.cell_tree_data[idx + i];
            if let Some(&subrect) = ctx.atlas_definition.tree_sub_rects.get(&cell_tree) {
                let x = i as f32 * dx + ctx.atlas_definition.tile_width as f32 / 2.0;
                let draw_command = DrawCommand {
                    tex: ctx.atlas_texture,
                    subrect,
                    x,
                    y: y + ctx.atlas_definition.tile_height as f32 / 2.0,
                    scale: 1.0,
                    drawing_extra: DrawCommandExtra::DrawWithPivot {
                        pivot: Pivot::Relative {
                            rel_x: 0.5,
                            rel_y: 0.95
                        }
                    },
                    sorting_layer: 0
                };
                ctx.scene_compositor.borrow_mut().enqueue(draw_command);
            }
        }
        y += dy;
    }
}
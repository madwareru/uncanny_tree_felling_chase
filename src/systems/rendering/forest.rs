use crate::core_subsystems::types::GlobalStorage;
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra, Pivot};

pub fn system(world: &mut hecs::World) {
    for (_, data) in world.query_mut::<(&mut GlobalStorage,)>() {
        let global_storage: &mut GlobalStorage = data.0;

        let mut y = 0.0;
        let dy = global_storage.atlas_definition.tile_height as f32;
        let dx = global_storage.atlas_definition.tile_width as f32;

        for idx in (0..global_storage.tilemap.w*global_storage.tilemap.h)
            .step_by(global_storage.tilemap.w)
        {
            for i in 0..global_storage.tilemap.w {
                let corner_tree = global_storage.forest.corner_tree_data[idx + i];
                if let Some(&subrect) = global_storage.atlas_definition.tree_sub_rects.get(&corner_tree) {
                    let x = i as f32 * dx;
                    let draw_command = DrawCommand {
                        tex: global_storage.atlas_texture,
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
                    global_storage.scene_compositor.enqueue(draw_command);
                }

                let cell_tree = global_storage.forest.cell_tree_data[idx + i];
                if let Some(&subrect) = global_storage.atlas_definition.tree_sub_rects.get(&cell_tree) {
                    let x = i as f32 * dx + global_storage.atlas_definition.tile_width as f32 / 2.0;
                    let draw_command = DrawCommand {
                        tex: global_storage.atlas_texture,
                        subrect,
                        x,
                        y: y + global_storage.atlas_definition.tile_height as f32 / 2.0,
                        scale: 1.0,
                        drawing_extra: DrawCommandExtra::DrawWithPivot {
                            pivot: Pivot::Relative {
                                rel_x: 0.5,
                                rel_y: 0.95
                            }
                        },
                        sorting_layer: 0
                    };
                    global_storage.scene_compositor.enqueue(draw_command);
                }
            }
            y += dy;
        }
    }
}
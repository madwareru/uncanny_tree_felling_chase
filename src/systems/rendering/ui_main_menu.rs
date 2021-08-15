use crate::core_subsystems::types::GlobalStorage;
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra, Pivot};
use crate::systems::rendering::ui_shared::{render_ui_background, render_idle_button_background};

pub fn system(world: &mut hecs::World) {
    for (_, data) in world.query_mut::<(&mut GlobalStorage,)>() {
        let global_storage: &mut GlobalStorage = data.0;
        render_ui_background(
            global_storage,
            20, global_storage.tilemap.w - 21,
            13, 24
        );
        render_idle_button_background(
            global_storage,
            22, global_storage.tilemap.w - 23,
            15, 18
        );
        let draw_command = DrawCommand {
            tex: global_storage.ui_atlas_texture,
            subrect: global_storage.atlas_definition.play_button_subrect,
            x: ((global_storage.tilemap.w / 2) * global_storage.atlas_definition.tile_width) as f32,
            y: (17 * global_storage.atlas_definition.tile_height) as f32,
            scale: 2.0,
            drawing_extra: DrawCommandExtra::DrawWithPivot {
                pivot: Pivot::Relative {rel_x: 0.5, rel_y: 0.5 }
            },
            sorting_layer: 3
        };
        global_storage.scene_compositor.enqueue(draw_command);
        render_idle_button_background(
            global_storage,
            22, global_storage.tilemap.w - 23,
            19, 22
        );
        let draw_command = DrawCommand {
            tex: global_storage.ui_atlas_texture,
            subrect: global_storage.atlas_definition.exit_button_subrect,
            x: ((global_storage.tilemap.w / 2) * global_storage.atlas_definition.tile_width) as f32,
            y: (21 * global_storage.atlas_definition.tile_height) as f32,
            scale: 2.0,
            drawing_extra: DrawCommandExtra::DrawWithPivot {
                pivot: Pivot::Relative {rel_x: 0.5, rel_y: 0.5 }
            },
            sorting_layer: 3
        };
        global_storage.scene_compositor.enqueue(draw_command);
    }
}
use crate::core_subsystems::types::GlobalStorage;
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra, Pivot};

pub fn system(world: &mut hecs::World) {
    for (_, data) in world.query_mut::<(&mut GlobalStorage,)>() {
        let global_storage: &mut GlobalStorage = data.0;
        render_borders(global_storage);
        tile_outer_borders(global_storage);
        render_title(global_storage);
    }
}

fn render_title(global_storage: &mut GlobalStorage) {
    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.game_title_subrect,
        x: (global_storage.atlas_definition.tile_width * (global_storage.tilemap.w / 2)) as f32,
        y: -1.75 * (global_storage.atlas_definition.tile_height as f32),
        scale: 2.0,
        drawing_extra: DrawCommandExtra::DrawWithPivot {
            pivot: Pivot::Relative { rel_x: 0.5, rel_y: 0.5 }
        },
        sorting_layer: 3
    };
    global_storage.scene_compositor.enqueue(draw_command);
}

fn render_borders(global_storage: &mut GlobalStorage) {
    let y_top = 0.0;
    let y_bottom = (global_storage.atlas_definition.tile_height * (global_storage.tilemap.h - 1)) as f32;
    let x_left = 0.0;
    let x_right = (global_storage.atlas_definition.tile_width * (global_storage.tilemap.w - 1)) as f32;

    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.ui_borders_3x3_0,
        x: x_left,
        y: y_top,
        scale: 1.0,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    global_storage.scene_compositor.enqueue(draw_command);

    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.ui_borders_3x3_6,
        x: x_left,
        y: y_bottom,
        scale: 1.0,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    global_storage.scene_compositor.enqueue(draw_command);

    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.ui_borders_3x3_2,
        x: x_right,
        y: y_top,
        scale: 1.0,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    global_storage.scene_compositor.enqueue(draw_command);

    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.ui_borders_3x3_8,
        x: x_right,
        y: y_bottom,
        scale: 1.0,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    global_storage.scene_compositor.enqueue(draw_command);

    for i in 1..global_storage.tilemap.w - 1 {
        let draw_command = DrawCommand {
            tex: global_storage.ui_atlas_texture,
            subrect: global_storage.atlas_definition.ui_borders_3x3_1,
            x: (global_storage.atlas_definition.tile_width * i) as f32,
            y: y_top,
            scale: 1.0,
            drawing_extra: DrawCommandExtra::Draw,
            sorting_layer: 2
        };
        global_storage.scene_compositor.enqueue(draw_command);

        let draw_command = DrawCommand {
            tex: global_storage.ui_atlas_texture,
            subrect: global_storage.atlas_definition.ui_borders_3x3_7,
            x: (global_storage.atlas_definition.tile_width * i) as f32,
            y: y_bottom,
            scale: 1.0,
            drawing_extra: DrawCommandExtra::Draw,
            sorting_layer: 2
        };
        global_storage.scene_compositor.enqueue(draw_command);
    }

    for j in 1..global_storage.tilemap.h - 1 {
        let draw_command = DrawCommand {
            tex: global_storage.ui_atlas_texture,
            subrect: global_storage.atlas_definition.ui_borders_3x3_3,
            x: x_left,
            y: (global_storage.atlas_definition.tile_height * j) as f32,
            scale: 1.0,
            drawing_extra: DrawCommandExtra::Draw,
            sorting_layer: 2
        };
        global_storage.scene_compositor.enqueue(draw_command);

        let draw_command = DrawCommand {
            tex: global_storage.ui_atlas_texture,
            subrect: global_storage.atlas_definition.ui_borders_3x3_5,
            x: x_right,
            y: (global_storage.atlas_definition.tile_height * j) as f32,
            scale: 1.0,
            drawing_extra: DrawCommandExtra::Draw,
            sorting_layer: 2
        };
        global_storage.scene_compositor.enqueue(draw_command);
    }
}

// Dirty but we have no time to do it in other manner
fn tile_outer_borders(global_storage: &mut GlobalStorage) {
    let y_top = 0.0;
    let y_bottom = (global_storage.atlas_definition.tile_height * (global_storage.tilemap.h)) as f32;
    let x_left = 0.0;
    let x_right = (global_storage.atlas_definition.tile_width * (global_storage.tilemap.w)) as f32;

    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.ui_background_box_3x3_4,
        x: x_left,
        y: y_top,
        scale: global_storage.tilemap.h as f32,
        drawing_extra: DrawCommandExtra::DrawWithPivot {
            pivot: Pivot::Relative {rel_x: 1.0, rel_y: 0.0}
        },
        sorting_layer: 2
    };
    global_storage.scene_compositor.enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.ui_background_box_3x3_4,
        x: x_left,
        y: y_top,
        scale: global_storage.tilemap.w as f32,
        drawing_extra: DrawCommandExtra::DrawWithPivot {
            pivot: Pivot::Relative {rel_x: 0.0, rel_y: 1.0}
        },
        sorting_layer: 2
    };
    global_storage.scene_compositor.enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.ui_background_box_3x3_4,
        x: x_right,
        y: y_top,
        scale: global_storage.tilemap.w as f32,
        drawing_extra: DrawCommandExtra::DrawWithPivot {
            pivot: Pivot::Relative {rel_x: 0.0, rel_y: 1.0}
        },
        sorting_layer: 2
    };
    global_storage.scene_compositor.enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.ui_background_box_3x3_4,
        x: x_right,
        y: y_top,
        scale: global_storage.tilemap.h as f32,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    global_storage.scene_compositor.enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.ui_background_box_3x3_4,
        x: x_left,
        y: y_top,
        scale: global_storage.tilemap.w as f32,
        drawing_extra: DrawCommandExtra::DrawWithPivot {
            pivot: Pivot::Relative {rel_x: 1.0, rel_y: 1.0}
        },
        sorting_layer: 2
    };
    global_storage.scene_compositor.enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.ui_background_box_3x3_4,
        x: x_left,
        y: y_bottom,
        scale: global_storage.tilemap.w as f32,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    global_storage.scene_compositor.enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.ui_background_box_3x3_4,
        x: x_right,
        y: y_bottom,
        scale: global_storage.tilemap.w as f32,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    global_storage.scene_compositor.enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: global_storage.ui_atlas_texture,
        subrect: global_storage.atlas_definition.ui_background_box_3x3_4,
        x: x_left,
        y: y_bottom,
        scale: global_storage.tilemap.w as f32,
        drawing_extra: DrawCommandExtra::DrawWithPivot {
            pivot: Pivot::Relative {rel_x: 1.0, rel_y: 0.0}
        },
        sorting_layer: 2
    };
    global_storage.scene_compositor.enqueue(draw_command);
}
use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra, Pivot, SceneCompositor};

pub fn system(ctx: &GlobalContext) {
    render_borders(ctx);
    tile_outer_borders(ctx);
    render_title(ctx);
}

fn render_title(ctx: &GlobalContext) {
    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.game_title_subrect,
        x: (ctx.atlas_definition.tile_width * (ctx.tilemap.w / 2)) as f32,
        y: -1.75 * (ctx.atlas_definition.tile_height as f32),
        scale: 2.0,
        drawing_extra: DrawCommandExtra::DrawWithPivot {
            pivot: Pivot::Relative { rel_x: 0.5, rel_y: 0.5 }
        },
        sorting_layer: 3
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);
}

fn render_borders(ctx: &GlobalContext) {
    let y_top = 0.0;
    let y_bottom = (ctx.atlas_definition.tile_height * (ctx.tilemap.h - 1)) as f32;
    let x_left = 0.0;
    let x_right = (ctx.atlas_definition.tile_width * (ctx.tilemap.w - 1)) as f32;

    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.ui_borders_3x3_0,
        x: x_left,
        y: y_top,
        scale: 1.0,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);

    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.ui_borders_3x3_6,
        x: x_left,
        y: y_bottom,
        scale: 1.0,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);

    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.ui_borders_3x3_2,
        x: x_right,
        y: y_top,
        scale: 1.0,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);

    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.ui_borders_3x3_8,
        x: x_right,
        y: y_bottom,
        scale: 1.0,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);

    for i in 1..ctx.tilemap.w - 1 {
        let draw_command = DrawCommand {
            tex: ctx.ui_atlas_texture,
            subrect: ctx.atlas_definition.ui_borders_3x3_1,
            x: (ctx.atlas_definition.tile_width * i) as f32,
            y: y_top,
            scale: 1.0,
            drawing_extra: DrawCommandExtra::Draw,
            sorting_layer: 2
        };
        ctx.scene_compositor.borrow_mut().enqueue(draw_command);

        let draw_command = DrawCommand {
            tex: ctx.ui_atlas_texture,
            subrect: ctx.atlas_definition.ui_borders_3x3_7,
            x: (ctx.atlas_definition.tile_width * i) as f32,
            y: y_bottom,
            scale: 1.0,
            drawing_extra: DrawCommandExtra::Draw,
            sorting_layer: 2
        };
        ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    }

    for j in 1..ctx.tilemap.h - 1 {
        let draw_command = DrawCommand {
            tex: ctx.ui_atlas_texture,
            subrect: ctx.atlas_definition.ui_borders_3x3_3,
            x: x_left,
            y: (ctx.atlas_definition.tile_height * j) as f32,
            scale: 1.0,
            drawing_extra: DrawCommandExtra::Draw,
            sorting_layer: 2
        };
        ctx.scene_compositor.borrow_mut().enqueue(draw_command);

        let draw_command = DrawCommand {
            tex: ctx.ui_atlas_texture,
            subrect: ctx.atlas_definition.ui_borders_3x3_5,
            x: x_right,
            y: (ctx.atlas_definition.tile_height * j) as f32,
            scale: 1.0,
            drawing_extra: DrawCommandExtra::Draw,
            sorting_layer: 2
        };
        ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    }
}

// Dirty but we have no time to do it in other manner
fn tile_outer_borders(ctx: &GlobalContext) {
    let y_top = 0.0;
    let y_bottom = (ctx.atlas_definition.tile_height * (ctx.tilemap.h)) as f32;
    let x_left = 0.0;
    let x_right = (ctx.atlas_definition.tile_width * (ctx.tilemap.w)) as f32;

    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.ui_background_box_3x3_4,
        x: x_left,
        y: y_top,
        scale: ctx.tilemap.h as f32,
        drawing_extra: DrawCommandExtra::DrawWithPivot {
            pivot: Pivot::Relative {rel_x: 1.0, rel_y: 0.0}
        },
        sorting_layer: 2
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.ui_background_box_3x3_4,
        x: x_left,
        y: y_top,
        scale: ctx.tilemap.w as f32,
        drawing_extra: DrawCommandExtra::DrawWithPivot {
            pivot: Pivot::Relative {rel_x: 0.0, rel_y: 1.0}
        },
        sorting_layer: 2
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.ui_background_box_3x3_4,
        x: x_right,
        y: y_top,
        scale: ctx.tilemap.w as f32,
        drawing_extra: DrawCommandExtra::DrawWithPivot {
            pivot: Pivot::Relative {rel_x: 0.0, rel_y: 1.0}
        },
        sorting_layer: 2
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.ui_background_box_3x3_4,
        x: x_right,
        y: y_top,
        scale: ctx.tilemap.h as f32,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.ui_background_box_3x3_4,
        x: x_left,
        y: y_top,
        scale: ctx.tilemap.w as f32,
        drawing_extra: DrawCommandExtra::DrawWithPivot {
            pivot: Pivot::Relative {rel_x: 1.0, rel_y: 1.0}
        },
        sorting_layer: 2
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.ui_background_box_3x3_4,
        x: x_left,
        y: y_bottom,
        scale: ctx.tilemap.w as f32,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.ui_background_box_3x3_4,
        x: x_right,
        y: y_bottom,
        scale: ctx.tilemap.w as f32,
        drawing_extra: DrawCommandExtra::Draw,
        sorting_layer: 2
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    let draw_command = DrawCommand {
        tex: ctx.ui_atlas_texture,
        subrect: ctx.atlas_definition.ui_background_box_3x3_4,
        x: x_left,
        y: y_bottom,
        scale: ctx.tilemap.w as f32,
        drawing_extra: DrawCommandExtra::DrawWithPivot {
            pivot: Pivot::Relative {rel_x: 1.0, rel_y: 0.0}
        },
        sorting_layer: 2
    };
    ctx.scene_compositor.borrow_mut().enqueue(draw_command);
}
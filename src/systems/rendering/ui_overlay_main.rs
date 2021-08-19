use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::atlas_serialization::UiTile;
use macro_tiler::atlas::rect_handle::{Having, DrawPivot, DrawSizeOverride};

pub fn system(ctx: &GlobalContext) {
    render_borders(ctx);
    tile_outer_borders(ctx);
    render_title(ctx);
}

fn render_title(ctx: &GlobalContext) {
    let handle = ctx.ui_atlas.acquire_handle(&UiTile::GameTitle).unwrap();
    let draw_command = macro_tiler::atlas::draw_command::builder()
        .having(DrawPivot::Relative([0.5, 0.5].into()))
        .having(DrawSizeOverride::ScaledUniform(2.0))
        .build(
            handle,
            (ctx.atlas_scheme.tile_width * (ctx.tilemap.borrow().w / 2)) as f32,
            -1.75 * (ctx.atlas_scheme.tile_height as f32)
        );
    ctx.scene_compositor.borrow_mut().enqueue(4, draw_command);
}

fn render_borders(ctx: &GlobalContext) {
    let y_top = 0.0;
    let y_bottom = (ctx.atlas_scheme.tile_height * (ctx.tilemap.borrow().h - 1)) as f32;
    let x_left = 0.0;
    let x_right = (ctx.atlas_scheme.tile_width * (ctx.tilemap.borrow().w - 1)) as f32;

    // todo
    // let draw_command = DrawCommand {
    //     tex: ctx.ui_atlas_texture,
    //     subrect: ctx.atlas_definition.ui_borders_3x3_0,
    //     x: x_left,
    //     y: y_top,
    //     scale: 1.0,
    //     drawing_extra: DrawCommandExtra::Draw,
    //     sorting_layer: 2
    // };
    // ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    //
    // let draw_command = DrawCommand {
    //     tex: ctx.ui_atlas_texture,
    //     subrect: ctx.atlas_definition.ui_borders_3x3_6,
    //     x: x_left,
    //     y: y_bottom,
    //     scale: 1.0,
    //     drawing_extra: DrawCommandExtra::Draw,
    //     sorting_layer: 2
    // };
    // ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    //
    // let draw_command = DrawCommand {
    //     tex: ctx.ui_atlas_texture,
    //     subrect: ctx.atlas_definition.ui_borders_3x3_2,
    //     x: x_right,
    //     y: y_top,
    //     scale: 1.0,
    //     drawing_extra: DrawCommandExtra::Draw,
    //     sorting_layer: 2
    // };
    // ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    //
    // let draw_command = DrawCommand {
    //     tex: ctx.ui_atlas_texture,
    //     subrect: ctx.atlas_definition.ui_borders_3x3_8,
    //     x: x_right,
    //     y: y_bottom,
    //     scale: 1.0,
    //     drawing_extra: DrawCommandExtra::Draw,
    //     sorting_layer: 2
    // };
    // ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    //
    // for i in 1..ctx.tilemap.borrow().w - 1 {
    //     let draw_command = DrawCommand {
    //         tex: ctx.ui_atlas_texture,
    //         subrect: ctx.atlas_definition.ui_borders_3x3_1,
    //         x: (ctx.atlas_definition.tile_width * i) as f32,
    //         y: y_top,
    //         scale: 1.0,
    //         drawing_extra: DrawCommandExtra::Draw,
    //         sorting_layer: 2
    //     };
    //     ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    //
    //     let draw_command = DrawCommand {
    //         tex: ctx.ui_atlas_texture,
    //         subrect: ctx.atlas_definition.ui_borders_3x3_7,
    //         x: (ctx.atlas_definition.tile_width * i) as f32,
    //         y: y_bottom,
    //         scale: 1.0,
    //         drawing_extra: DrawCommandExtra::Draw,
    //         sorting_layer: 2
    //     };
    //     ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    // }
    //
    // for j in 1..ctx.tilemap.borrow().h - 1 {
    //     let draw_command = DrawCommand {
    //         tex: ctx.ui_atlas_texture,
    //         subrect: ctx.atlas_definition.ui_borders_3x3_3,
    //         x: x_left,
    //         y: (ctx.atlas_definition.tile_height * j) as f32,
    //         scale: 1.0,
    //         drawing_extra: DrawCommandExtra::Draw,
    //         sorting_layer: 2
    //     };
    //     ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    //
    //     let draw_command = DrawCommand {
    //         tex: ctx.ui_atlas_texture,
    //         subrect: ctx.atlas_definition.ui_borders_3x3_5,
    //         x: x_right,
    //         y: (ctx.atlas_definition.tile_height * j) as f32,
    //         scale: 1.0,
    //         drawing_extra: DrawCommandExtra::Draw,
    //         sorting_layer: 2
    //     };
    //     ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    // }
}

// Dirty but we have no time to do it in other manner
fn tile_outer_borders(ctx: &GlobalContext) {
    let y_top = 0.0;
    let y_bottom = (ctx.atlas_scheme.tile_height * (ctx.tilemap.borrow().h)) as f32;
    let x_left = 0.0;
    let x_right = (ctx.atlas_scheme.tile_width * (ctx.tilemap.borrow().w)) as f32;

    // todo
    // let draw_command = DrawCommand {
    //     tex: ctx.ui_atlas_texture,
    //     subrect: ctx.atlas_definition.ui_background_box_3x3_4,
    //     x: x_left,
    //     y: y_top,
    //     scale: ctx.tilemap.borrow().h as f32,
    //     drawing_extra: DrawCommandExtra::DrawWithPivot {
    //         pivot: Pivot::Relative {rel_x: 1.0, rel_y: 0.0}
    //     },
    //     sorting_layer: 2
    // };
    // ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    // let draw_command = DrawCommand {
    //     tex: ctx.ui_atlas_texture,
    //     subrect: ctx.atlas_definition.ui_background_box_3x3_4,
    //     x: x_left,
    //     y: y_top,
    //     scale: ctx.tilemap.borrow().w as f32,
    //     drawing_extra: DrawCommandExtra::DrawWithPivot {
    //         pivot: Pivot::Relative {rel_x: 0.0, rel_y: 1.0}
    //     },
    //     sorting_layer: 2
    // };
    // ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    // let draw_command = DrawCommand {
    //     tex: ctx.ui_atlas_texture,
    //     subrect: ctx.atlas_definition.ui_background_box_3x3_4,
    //     x: x_right,
    //     y: y_top,
    //     scale: ctx.tilemap.borrow().w as f32,
    //     drawing_extra: DrawCommandExtra::DrawWithPivot {
    //         pivot: Pivot::Relative {rel_x: 0.0, rel_y: 1.0}
    //     },
    //     sorting_layer: 2
    // };
    // ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    // let draw_command = DrawCommand {
    //     tex: ctx.ui_atlas_texture,
    //     subrect: ctx.atlas_definition.ui_background_box_3x3_4,
    //     x: x_right,
    //     y: y_top,
    //     scale: ctx.tilemap.borrow().h as f32,
    //     drawing_extra: DrawCommandExtra::Draw,
    //     sorting_layer: 2
    // };
    // ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    // let draw_command = DrawCommand {
    //     tex: ctx.ui_atlas_texture,
    //     subrect: ctx.atlas_definition.ui_background_box_3x3_4,
    //     x: x_left,
    //     y: y_top,
    //     scale: ctx.tilemap.borrow().w as f32,
    //     drawing_extra: DrawCommandExtra::DrawWithPivot {
    //         pivot: Pivot::Relative {rel_x: 1.0, rel_y: 1.0}
    //     },
    //     sorting_layer: 2
    // };
    // ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    // let draw_command = DrawCommand {
    //     tex: ctx.ui_atlas_texture,
    //     subrect: ctx.atlas_definition.ui_background_box_3x3_4,
    //     x: x_left,
    //     y: y_bottom,
    //     scale: ctx.tilemap.borrow().w as f32,
    //     drawing_extra: DrawCommandExtra::Draw,
    //     sorting_layer: 2
    // };
    // ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    // let draw_command = DrawCommand {
    //     tex: ctx.ui_atlas_texture,
    //     subrect: ctx.atlas_definition.ui_background_box_3x3_4,
    //     x: x_right,
    //     y: y_bottom,
    //     scale: ctx.tilemap.borrow().w as f32,
    //     drawing_extra: DrawCommandExtra::Draw,
    //     sorting_layer: 2
    // };
    // ctx.scene_compositor.borrow_mut().enqueue(draw_command);
    // let draw_command = DrawCommand {
    //     tex: ctx.ui_atlas_texture,
    //     subrect: ctx.atlas_definition.ui_background_box_3x3_4,
    //     x: x_left,
    //     y: y_bottom,
    //     scale: ctx.tilemap.borrow().w as f32,
    //     drawing_extra: DrawCommandExtra::DrawWithPivot {
    //         pivot: Pivot::Relative {rel_x: 1.0, rel_y: 0.0}
    //     },
    //     sorting_layer: 2
    // };
    // ctx.scene_compositor.borrow_mut().enqueue(draw_command);
}
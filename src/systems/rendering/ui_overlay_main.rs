use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::atlas_serialization::UiTile;
use macro_tiler::atlas::rect_handle::{Having, DrawPivot, DrawSizeOverride};
use macro_tiler::atlas::draw_command::draw_command_builder;
use macroquad::prelude::{screen_width, screen_height};
use crate::core_subsystems::rendering::{UiRenderLayer};

pub fn system(ctx: &GlobalContext) {
    render_borders(ctx);
    tile_outer_borders(ctx);
    render_title(ctx);
}

fn render_title(ctx: &GlobalContext) {
    let handle = ctx.ui_atlas.acquire_handle(&UiTile::GameTitle).unwrap();
    let draw_command = macro_tiler::atlas::draw_command::draw_command_builder()
        .having(DrawPivot::Relative([0.5, 0.5].into()))
        .having(DrawSizeOverride::ScaledUniform(2.0))
        .build(
            handle,
            (ctx.atlas_scheme.tile_width * (ctx.tilemap.borrow().w / 2)) as f32,
            -1.75 * (ctx.atlas_scheme.tile_height as f32),
        );
    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Buttons, draw_command);
}

fn render_borders(ctx: &GlobalContext) {
    macro_rules! handle ( { $id:ident } => { ctx.ui_atlas.acquire_handle(&UiTile::$id).unwrap() });

    let y_top = 0.0;
    let y_bottom = (ctx.atlas_scheme.tile_height * (ctx.tilemap.borrow().h - 2)) as f32;
    let x_left = 0.0;
    let x_right = (ctx.atlas_scheme.tile_width * (ctx.tilemap.borrow().w - 2)) as f32;

    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Background, draw_command_builder()
        .having(DrawSizeOverride::ScaledUniform(2.0))
        .build(handle!(UiBorders3x3_0), x_left, y_top),
    );
    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Background, draw_command_builder()
        .having(DrawSizeOverride::ScaledUniform(2.0))
        .build(handle!(UiBorders3x3_2), x_right, y_top),
    );
    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Background, draw_command_builder()
        .having(DrawSizeOverride::ScaledUniform(2.0))
        .build(handle!(UiBorders3x3_6), x_left, y_bottom),
    );
    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Background, draw_command_builder()
        .having(DrawSizeOverride::ScaledUniform(2.0))
        .build(handle!(UiBorders3x3_8), x_right, y_bottom),
    );

    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Background, draw_command_builder()
        .having(
            DrawSizeOverride::ScaledNonUniform(
                [(ctx.tilemap.borrow().w - 4) as f32, 2.0].into()
            )
        )
        .build(handle!(UiBorders3x3_1), ctx.atlas_scheme.tile_width as f32 * 2.0, y_top),
    );
    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Background, draw_command_builder()
        .having(
            DrawSizeOverride::ScaledNonUniform(
                [(ctx.tilemap.borrow().w - 4) as f32, 2.0].into()
            )
        )
        .build(handle!(UiBorders3x3_7), ctx.atlas_scheme.tile_width as f32 * 2.0, y_bottom),
    );

    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Background, draw_command_builder()
        .having(
            DrawSizeOverride::ScaledNonUniform(
                [2.0, (ctx.tilemap.borrow().h - 4) as f32].into()
            )
        )
        .build(handle!(UiBorders3x3_3), x_left, ctx.atlas_scheme.tile_height as f32 * 2.0),
    );
    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Background, draw_command_builder()
        .having(
            DrawSizeOverride::ScaledNonUniform(
                [2.0, (ctx.tilemap.borrow().h - 4) as f32].into()
            )
        )
        .build(handle!(UiBorders3x3_5), x_right, ctx.atlas_scheme.tile_height as f32 * 2.0),
    );
}

fn tile_outer_borders(ctx: &GlobalContext) {
    macro_rules! handle ( { $id:ident } => { ctx.ui_atlas.acquire_handle(&UiTile::$id).unwrap() });

    let dpi_scale = macro_tiler::utility::with_context(|ctx| ctx.dpi_scale());
    let true_tile_w = ctx.draw_scale * ctx.atlas_scheme.tile_width as f32;
    let true_tile_h = ctx.draw_scale * ctx.atlas_scheme.tile_height as f32;

    let sw = screen_width() / dpi_scale;
    let sh = screen_height() / dpi_scale;

    let screen_width_in_tiles = sw / true_tile_w;
    let screen_height_in_tiles = sh / true_tile_h;
    let h_scale = (screen_height_in_tiles - ctx.tilemap.borrow().h as f32) / 2.0;
    let w_scale = (screen_width_in_tiles - ctx.tilemap.borrow().w as f32) / 2.0;

    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Background, draw_command_builder()
        .having(
            DrawSizeOverride::ScaledNonUniform(
                [
                    w_scale,
                    ctx.tilemap.borrow().h as f32 + h_scale
                ].into()
            )
        )
        .having(DrawPivot::BottomRight)
        .build(
            handle!(UiBackgroundBox3x3_4),
            0.0,
            (ctx.tilemap.borrow().h * ctx.atlas_scheme.tile_height) as f32,
        ),
    );
    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Background, draw_command_builder()
        .having(
            DrawSizeOverride::ScaledNonUniform(
                [
                    ctx.tilemap.borrow().w as f32 + w_scale,
                    h_scale
                ].into()
            )
        )
        .having(DrawPivot::BottomLeft)
        .build(handle!(UiBackgroundBox3x3_4), 0.0, 0.0),
    );
    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Background, draw_command_builder()
        .having(
            DrawSizeOverride::ScaledNonUniform(
                [
                    w_scale,
                    ctx.tilemap.borrow().h as f32 + h_scale
                ].into()
            )
        )
        .build(
            handle!(UiBackgroundBox3x3_4),
            (ctx.tilemap.borrow().w * ctx.atlas_scheme.tile_width) as f32,
            0.0,
        ),
    );
    ctx.scene_compositor.borrow_mut().enqueue_ui(UiRenderLayer::Background, draw_command_builder()
        .having(
            DrawSizeOverride::ScaledNonUniform(
                [
                    ctx.tilemap.borrow().w as f32 + w_scale,
                    h_scale
                ].into()
            )
        )
        .having(DrawPivot::TopRight)
        .build(
            handle!(UiBackgroundBox3x3_4),
            (ctx.tilemap.borrow().w * ctx.atlas_scheme.tile_width) as f32,
            (ctx.tilemap.borrow().h * ctx.atlas_scheme.tile_height) as f32,
        ),
    );
}
use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::atlas_serialization::{UiTile};
use crate::components::UiRect;
use macro_tiler::atlas::rect_handle::{AtlasRectHandle, Having, DrawSizeOverride};
use crate::core_subsystems::rendering::{UiRenderLayer};
use macro_tiler::atlas::draw_command::draw_command_builder;

#[inline]
pub fn render_ui_selection(ctx: &GlobalContext, ui_rect: &UiRect) {
    render_box(ctx, ui_rect,
               &[
                   ctx.ui_atlas.acquire_handle(&UiTile::UiSelection3x3_0).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiSelection3x3_1).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiSelection3x3_2).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiSelection3x3_3).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiSelection3x3_4).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiSelection3x3_5).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiSelection3x3_6).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiSelection3x3_7).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiSelection3x3_8).unwrap()
               ],
               UiRenderLayer::ButtonSelection,
    );
}

#[inline]
pub fn render_ui_background(ctx: &GlobalContext, ui_rect: &UiRect) {
    render_box(ctx, ui_rect,
               &[
                   ctx.ui_atlas.acquire_handle(&UiTile::UiBackgroundBox3x3_0).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiBackgroundBox3x3_1).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiBackgroundBox3x3_2).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiBackgroundBox3x3_3).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiBackgroundBox3x3_4).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiBackgroundBox3x3_5).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiBackgroundBox3x3_6).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiBackgroundBox3x3_7).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::UiBackgroundBox3x3_8).unwrap()
               ],
               UiRenderLayer::Buttons,
    )
}

#[inline]
pub fn render_idle_button_background(ctx: &GlobalContext, ui_rect: &UiRect) {
    render_box(ctx, ui_rect,
               &[
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonIdle3x3_0).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonIdle3x3_1).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonIdle3x3_2).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonIdle3x3_3).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonIdle3x3_4).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonIdle3x3_5).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonIdle3x3_6).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonIdle3x3_7).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonIdle3x3_8).unwrap()
               ],
               UiRenderLayer::Buttons,
    )
}

#[inline]
pub fn render_hover_button_background(ctx: &GlobalContext, ui_rect: &UiRect) {
    render_box(ctx, ui_rect,
               &[
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonHover3x3_0).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonHover3x3_1).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonHover3x3_2).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonHover3x3_3).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonHover3x3_4).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonHover3x3_5).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonHover3x3_6).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonHover3x3_7).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonHover3x3_8).unwrap()
               ],
               UiRenderLayer::Buttons,
    )
}

#[inline]
pub fn render_clicked_button_background(ctx: &GlobalContext, ui_rect: &UiRect) {
    render_box(ctx, ui_rect,
               &[
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonClicked3x3_0).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonClicked3x3_1).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonClicked3x3_2).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonClicked3x3_3).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonClicked3x3_4).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonClicked3x3_5).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonClicked3x3_6).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonClicked3x3_7).unwrap(),
                   ctx.ui_atlas.acquire_handle(&UiTile::ButtonClicked3x3_8).unwrap()
               ],
               UiRenderLayer::Buttons,
    )
}

fn render_box(
    ctx: &GlobalContext,
    ui_rect: &UiRect,
    box_sub_tiles: &[AtlasRectHandle],
    render_layer: UiRenderLayer,
) {
    let &UiRect {
        top_left: (left_tile, top_tile),
        bottom_right: (right_tile, bottom_tile)
    } = ui_rect;
    for (i, j, scale_override) in &[
        (left_tile, top_tile, None),
        (right_tile, top_tile, None),
        (left_tile, bottom_tile, None),
        (right_tile, bottom_tile, None),
        (left_tile + 1, top_tile, Some(((right_tile - left_tile - 1) as f32, 1.0))),
        (left_tile + 1, bottom_tile, Some(((right_tile - left_tile - 1) as f32, 1.0))),
        (left_tile, top_tile + 1, Some((1.0, (bottom_tile - top_tile - 1) as f32))),
        (right_tile, top_tile + 1, Some((1.0, (bottom_tile - top_tile - 1) as f32))),
        (left_tile + 1, top_tile + 1, Some((
            (right_tile - left_tile - 1) as f32,
            (bottom_tile - top_tile - 1) as f32
        ))),
    ] {
        let (x, y) = (
            (ctx.atlas_scheme.tile_height as i32 * i) as f32,
            (ctx.atlas_scheme.tile_height as i32 * j) as f32
        );
        let handle = choose_handle(box_sub_tiles, left_tile, top_tile, right_tile, bottom_tile, *j, *i);
        let draw_command = match scale_override {
            Some((sx, sy)) => {
                draw_command_builder()
                    .having(DrawSizeOverride::ScaledNonUniform([*sx, *sy].into()))
                    .build(handle, x, y)
            }
            None => draw_command_builder().build(handle, x, y)
        };
        ctx.scene_compositor.borrow_mut().enqueue_ui(render_layer, draw_command);
    }
}

fn choose_handle(
    box_sub_tiles: &[AtlasRectHandle],
    left_tile: i32,
    top_tile: i32,
    right_tile: i32,
    bottom_tile: i32,
    j: i32,
    i: i32,
) -> AtlasRectHandle {
    match j {
        jt if jt == top_tile => {
            match i {
                t if t == left_tile => {
                    box_sub_tiles[0]
                }
                t if t == right_tile => {
                    box_sub_tiles[2]
                }
                _ => {
                    box_sub_tiles[1]
                }
            }
        }
        jt if jt == bottom_tile => {
            match i {
                t if t == left_tile => {
                    box_sub_tiles[6]
                }
                t if t == right_tile => {
                    box_sub_tiles[8]
                }
                _ => {
                    box_sub_tiles[7]
                }
            }
        }
        _ => {
            match i {
                t if t == left_tile => {
                    box_sub_tiles[3]
                }
                t if t == right_tile => {
                    box_sub_tiles[5]
                }
                _ => {
                    box_sub_tiles[4]
                }
            }
        }
    }
}
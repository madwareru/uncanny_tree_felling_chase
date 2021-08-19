use crate::core_subsystems::types::GlobalContext;
use macroquad::prelude::*;

#[inline]
pub fn peek_tile(global_ctx: &GlobalContext) -> (i32, i32) { peek_tile_ext(global_ctx, 1.0) }

pub fn peek_tile_ext(global_ctx: &GlobalContext, scale: f32) -> (i32, i32) {
    let dpi_scale = macro_tiler::utility::with_context(|ctx| ctx.dpi_scale());

    let (mouse_x, mouse_y) = mouse_position();
    let (mouse_x, mouse_y) = (mouse_x / dpi_scale, mouse_y / dpi_scale);

    let true_tile_w = scale * global_ctx.draw_scale * global_ctx.atlas_scheme.tile_width as f32;
    let true_tile_h = scale * global_ctx.draw_scale * global_ctx.atlas_scheme.tile_height as f32;

    let (tile_x, tile_y) = (
        (mouse_x / true_tile_w),
        (mouse_y / true_tile_h)
    );

    let screen_width_in_tiles = screen_width() / dpi_scale / true_tile_w;
    let screen_height_in_tiles = screen_height() / dpi_scale / true_tile_h;

    (
        (tile_x - (screen_width_in_tiles - global_ctx.tilemap.borrow().w as f32 / scale) / 2.0).trunc() as i32,
        (tile_y - (screen_height_in_tiles - global_ctx.tilemap.borrow().h as f32 / scale) / 2.0).trunc() as i32
    )
}
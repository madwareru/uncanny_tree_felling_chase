use crate::core_subsystems::types::GlobalContext;
use macroquad::prelude::*;
use crate::core_subsystems::atlas_serialization::AtlasDefinition;
use std::sync::Arc;

pub fn peek_tile(global_ctx: &GlobalContext) -> (i32, i32) {
    let InternalGlContext {
        quad_context: ctx, ..
    } = unsafe { get_internal_gl() };

    let (mouse_x, mouse_y) = mouse_position();
    let (mouse_x, mouse_y) = (mouse_x / ctx.dpi_scale(), mouse_y / ctx.dpi_scale());

    let true_tile_w = global_ctx.draw_scale * global_ctx.atlas_definition.tile_width as f32;
    let true_tile_h = global_ctx.draw_scale * global_ctx.atlas_definition.tile_height as f32;

    let (tile_x, tile_y) = (
        (mouse_x / true_tile_w),
        (mouse_y / true_tile_h)
    );

    let screen_width_in_tiles = screen_width() / ctx.dpi_scale() / true_tile_w;
    let screen_height_in_tiles = screen_height() / ctx.dpi_scale() / true_tile_h;

    (
        (tile_x - (screen_width_in_tiles - global_ctx.tilemap.w as f32) / 2.0).trunc() as i32,
        (tile_y - (screen_height_in_tiles - global_ctx.tilemap.h as f32) / 2.0).trunc() as i32
    )
}
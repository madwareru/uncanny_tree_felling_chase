use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::atlas_serialization::{TreeType, UiTile};
use crate::core_subsystems::peek_utils::{peek_tile_ext};
use crate::core_subsystems::passability_utils::is_half_tile_passable;
use macro_tiler::atlas::rect_handle::{Having, DrawSizeOverride, DrawPivot};

pub fn system(ctx: &GlobalContext) {
    let half_tile_w = ctx.atlas_scheme.tile_width / 2;
    let half_tile_h = ctx.atlas_scheme.tile_height / 2;

    for j in 0..ctx.tilemap.borrow().h * 2 {
        for i in 0..ctx.tilemap.borrow().w * 2 {
            let is_passable = is_half_tile_passable(ctx, i, j);
            let is_blocked = !is_passable || {
                let idx = (j / 2) * ctx.tilemap.borrow().w + i / 2;
                ctx.forest.borrow().corner_tree_data[idx] != TreeType::None ||
                    ctx.forest.borrow().cell_tree_data[idx] != TreeType::None
            };

            let (peek_x, peek_y) = peek_tile_ext(ctx, 0.5);
            let x = (i as f32 + 0.5) * half_tile_w as f32;
            let y = (j as f32 + 0.5) * half_tile_h as f32;
            let handle = ctx.ui_atlas.acquire_handle(
                if is_blocked {
                    &UiTile::BlockedCell
                } else {
                    if peek_x == (i as i32) && peek_y == (j as i32) {
                        &UiTile::SelectedCell
                    } else {
                        &UiTile::OkCell
                    }
                }
            ).unwrap();
            let scale = if peek_x == (i as i32) && peek_y == (j as i32) {
                0.5
            } else {
                match (peek_x - i as i32).abs().max((peek_y - j as i32).abs()) {
                    1 => 0.42,
                    2 => 0.36,
                    _ => 0.25
                }
            };

            let draw_command = macro_tiler::atlas::draw_command::builder()
                .having(DrawSizeOverride::ScaledUniform(scale))
                .having(DrawPivot::Relative([0.5, 0.5].into()))
                .build(handle, x, y);
            ctx.scene_compositor.borrow_mut().enqueue(2, draw_command);
        }
    }
}
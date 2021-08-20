use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::atlas_serialization::{TreeType, UiTile};
use crate::core_subsystems::peek_utils::{peek_tile_ext};
use crate::core_subsystems::passability_utils::is_half_tile_passable;
use macro_tiler::atlas::rect_handle::{Having, DrawSizeOverride, DrawPivot, DrawColorOverride};
use crate::core_subsystems::rendering::RenderLayer;

pub fn system(ctx: &GlobalContext) {
    let half_tile_w = ctx.atlas_scheme.tile_width / 2;
    let half_tile_h = ctx.atlas_scheme.tile_height / 2;
    let (peek_x, peek_y) = peek_tile_ext(ctx, 0.5);

    for j in 2..(ctx.tilemap.borrow().h - 1) * 2 {
        for i in 2..(ctx.tilemap.borrow().w - 1) * 2 {
            let is_passable = is_half_tile_passable(ctx, i, j);
            let is_blocked = !is_passable || {
                let idx = (j / 2) * ctx.tilemap.borrow().w + i / 2;
                let corner_idx = match (i % 2, j % 2) {
                    (0, 0) => idx,
                    (0, _) => idx + ctx.tilemap.borrow().w,
                    (_, 0) => idx + 1,
                    _ => idx + ctx.tilemap.borrow().w + 1
                };
                let forest = ctx.forest.borrow();
                forest.corner_tree_data[corner_idx] != TreeType::None ||
                    forest.cell_tree_data[idx] != TreeType::None
            };

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
            let dx = (peek_x - i as i32) as f32;
            let dy = (peek_y - j as i32) as f32;
            let sqr_dist = (dx * dx + dy * dy).powf(0.25).clamp(2.35, 6.0);

            let scale = 1.0 / sqr_dist;

            let draw_command = macro_tiler::atlas::draw_command::draw_command_builder()
                .having(DrawSizeOverride::ScaledUniform(scale))
                .having(DrawPivot::Relative([0.5, 0.5].into()))
                .having(DrawColorOverride::Alpha((scale * 2.4).clamp(0.35, 0.7)))
                .build(handle, x, y);
            ctx.scene_compositor.borrow_mut().enqueue(RenderLayer::TileMapOverlay, draw_command);
        }
    }
}
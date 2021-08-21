use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::atlas_serialization::{TreeType, UiTile};
use crate::core_subsystems::peek_utils::{peek_tile_ext};
use crate::core_subsystems::passability_utils::is_half_tile_passable;
use macro_tiler::atlas::rect_handle::{Having, DrawSizeOverride, DrawPivot, DrawColorOverride};
use crate::core_subsystems::rendering::RenderLayer;
use crate::core_subsystems::player_landing_info::MapFieldOccupation;
use macroquad::color::Color;

pub fn system(ctx: &GlobalContext) {
    let half_tile_w = ctx.atlas_scheme.tile_width / 2;
    let half_tile_h = ctx.atlas_scheme.tile_height / 2;
    let (peek_x, peek_y) = peek_tile_ext(ctx, 0.5);

    let left = (peek_x - 6).clamp(2, (ctx.tilemap.borrow().w - 1) as i32 * 2) as usize;
    let right = (peek_x + 6).clamp(2, (ctx.tilemap.borrow().w - 1) as i32 * 2) as usize;
    let top = (peek_y - 6).clamp(2, (ctx.tilemap.borrow().h - 1) as i32 * 2) as usize;
    let bottom = (peek_y + 6).clamp(2, (ctx.tilemap.borrow().h - 1) as i32 * 2) as usize;

    for j in top..bottom {
        for i in left..right {
            let occupation_data = ctx.landing_occupation_data.borrow().get(i, j);
            let (handle, minion_data) = match occupation_data {
                MapFieldOccupation::Unoccupied =>
                    (
                        ctx.ui_atlas.acquire_handle(
                            if peek_x == (i as i32) && peek_y == (j as i32) {
                                &UiTile::SelectedCell
                            } else {
                                &UiTile::OkCell
                            }
                        ).unwrap()
                        , None
                    ),
                MapFieldOccupation::Blocked =>
                    (ctx.ui_atlas.acquire_handle(&UiTile::BlockedCell).unwrap(), None),
                MapFieldOccupation::MinionLanded { is_big, .. } =>
                    (ctx.ui_atlas.acquire_handle(&UiTile::SelectedCell).unwrap(), Some(is_big)),
            };

            let x = (i as f32 + 0.5) * half_tile_w as f32;
            let y = (j as f32 + 0.5) * half_tile_h as f32;

            let scale = if minion_data.is_none() {
                let dx = (peek_x - i as i32) as f32;
                let dy = (peek_y - j as i32) as f32;
                let sqr_dist = (dx * dx + dy * dy).powf(0.5).max(2.35);
                1.0 / sqr_dist
            } else {
                0.45
            };
            if scale < 0.175 {
                continue;
            }

            let alpha = (scale * 2.4).clamp(0.35, 0.7);

            let draw_command = macro_tiler::atlas::draw_command::draw_command_builder()
                .having(DrawSizeOverride::ScaledUniform( scale ))
                .having(DrawPivot::Relative([0.5, 0.5].into()))
                .having(match minion_data {
                    None => DrawColorOverride::Alpha(alpha),
                    Some(is_big) => {
                        if is_big {
                            DrawColorOverride::Tint(Color::new(0.0, 1.0, 0.0, alpha))
                        } else {
                            DrawColorOverride::Tint(Color::new(0.0, 0.8, 0.3, alpha))
                        }
                    }
                })
                .build(handle, x, y);
            ctx.scene_compositor.borrow_mut().enqueue(RenderLayer::TileMapOverlay, draw_command);
        }
    }
}
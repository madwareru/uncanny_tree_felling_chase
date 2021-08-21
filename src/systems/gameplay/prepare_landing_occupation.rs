use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::atlas_serialization::{TreeType, UiTile};
use crate::core_subsystems::peek_utils::{peek_tile_ext};
use crate::core_subsystems::passability_utils::is_half_tile_passable;
use macro_tiler::atlas::rect_handle::{Having, DrawSizeOverride, DrawPivot, DrawColorOverride};
use crate::core_subsystems::rendering::RenderLayer;
use crate::core_subsystems::player_landing_info::MapFieldOccupation;

pub fn system(ctx: &GlobalContext) {
    ctx.landing_occupation_data.borrow_mut().clear();

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
                forest.corner_tree_data[corner_idx].tree_type != TreeType::None ||
                    forest.cell_tree_data[idx].tree_type != TreeType::None
            };
            if is_blocked {
                *ctx.landing_occupation_data.borrow_mut().take_mut(i, j) = MapFieldOccupation::Blocked;
            }
        }
    }
}
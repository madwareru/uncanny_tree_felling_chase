use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::atlas_serialization::{TreeType, MainTile};
use macro_tiler::atlas::rect_handle::{Having, DrawPivot, DrawColorOverride, AtlasRectHandle};
use crate::core_subsystems::rendering::RenderLayer;
use crate::core_subsystems::peek_utils::{peek_tile_ext};
use macro_tiler::atlas::draw_command::{draw_command_builder, DrawCommand};

pub fn system(ctx: &GlobalContext) {
    let mut y = 0.0;
    let dy = ctx.atlas_scheme.tile_height as f32;
    let dx = ctx.atlas_scheme.tile_width as f32;

    let (peek_x, peek_y) = peek_tile_ext(ctx, 0.5);
    let peek_x = peek_x as f32 * ctx.atlas_scheme.tile_width as f32 / 2.0;
    let peek_y = peek_y as f32 * ctx.atlas_scheme.tile_height as f32 / 2.0;

    for idx in (0..ctx.tilemap.borrow().w*ctx.tilemap.borrow().h)
        .step_by(ctx.tilemap.borrow().w)
    {
        for i in 0..ctx.tilemap.borrow().w {
            let corner_tree = ctx.forest.borrow().corner_tree_data[idx + i];
            let rect_handle = match corner_tree.tree_type {
                TreeType::None => None,
                some_tree => ctx.main_atlas.acquire_handle(
                    match some_tree {
                        TreeType::Pine => &MainTile::Pine,
                        TreeType::Oak => &MainTile::Oak,
                        TreeType::Bush => &MainTile::Bush,
                        TreeType::Stump => &MainTile::Stump,
                        _ => unreachable!()
                    }
                )
            };
            if let Some(handle) = rect_handle {
                let x = i as f32 * dx;
                let draw_command = make_draw_command(peek_x, peek_y, handle, x, y);
                ctx.scene_compositor.borrow_mut().enqueue(RenderLayer::MapObjects, draw_command);
            }

            let cell_tree = ctx.forest.borrow().cell_tree_data[idx + i];
            let rect_handle = match cell_tree.tree_type {
                TreeType::None => None,
                some_tree => ctx.main_atlas.acquire_handle(
                    match some_tree {
                        TreeType::Pine => &MainTile::Pine,
                        TreeType::Oak => &MainTile::Oak,
                        TreeType::Bush => &MainTile::Bush,
                        TreeType::Stump => &MainTile::Stump,
                        _ => unreachable!()
                    }
                )
            };
            if let Some(handle) = rect_handle {
                let x = i as f32 * dx + ctx.atlas_scheme.tile_width as f32 / 2.0;
                let y = y + ctx.atlas_scheme.tile_height as f32 / 2.0;
                let draw_command = make_draw_command(peek_x, peek_y, handle, x, y);
                ctx.scene_compositor.borrow_mut().enqueue(RenderLayer::MapObjects, draw_command);
            }
        }
        y += dy;
    }
}

fn make_draw_command(peek_x: f32, peek_y: f32, handle: AtlasRectHandle, x: f32, y: f32) -> DrawCommand {
    let alpha = {
        let (dx, dy) = (x - peek_x, y - peek_y);
        let distance = dx * dx + dy * dy;
        (distance / 200000.0).clamp(0.1, 1.0)
    };
    draw_command_builder()
        .having(DrawPivot::Relative([0.5, 0.9].into()))
        .having(DrawColorOverride::Alpha(alpha))
        .build(handle, x, y)
}
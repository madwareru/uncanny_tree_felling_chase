use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::atlas_serialization::{TreeType, MainTile};
use macro_tiler::atlas::rect_handle::{Having, DrawPivot};
use crate::core_subsystems::rendering::RenderLayer;

pub fn system(ctx: &GlobalContext) {
    let mut y = 0.0;
    let dy = ctx.atlas_scheme.tile_height as f32;
    let dx = ctx.atlas_scheme.tile_width as f32;

    for idx in (0..ctx.tilemap.borrow().w*ctx.tilemap.borrow().h)
        .step_by(ctx.tilemap.borrow().w)
    {
        for i in 0..ctx.tilemap.borrow().w {
            let corner_tree = ctx.forest.borrow().corner_tree_data[idx + i];
            let rect_handle = match corner_tree {
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
                let draw_command = macro_tiler::atlas::draw_command::draw_command_builder()
                    .having(DrawPivot::Relative([0.5, 0.9].into()))
                    .build(handle, x, y);
                ctx.scene_compositor.borrow_mut().enqueue(RenderLayer::MapObjects, draw_command);
            }

            let cell_tree = ctx.forest.borrow().cell_tree_data[idx + i];
            let rect_handle = match cell_tree {
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
                let draw_command = macro_tiler::atlas::draw_command::draw_command_builder()
                    .having(DrawPivot::Relative([0.5, 0.9].into()))
                    .build(handle, x, y + ctx.atlas_scheme.tile_height as f32 / 2.0);
                ctx.scene_compositor.borrow_mut().enqueue(RenderLayer::MapObjects, draw_command);
            }
        }
        y += dy;
    }
}
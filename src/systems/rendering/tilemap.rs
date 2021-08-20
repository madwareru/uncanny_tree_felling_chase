use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::rendering::RenderLayer;

pub fn system(ctx: &GlobalContext) {
    for j in 0..ctx.tilemap.borrow().h {
        for i in 0..ctx.tilemap.borrow().w {
            let idx = j * ctx.tilemap.borrow().w + i;
            let idx = ctx.tilemap.borrow().map_data[idx];
            let tile = ctx.tilemap.borrow().tiles[idx];
            let handle = ctx.main_atlas.acquire_handle(&tile).unwrap();

            let draw_command = macro_tiler::atlas::draw_command::draw_command_builder()
                .build(
                    handle,
                    (i * ctx.atlas_scheme.tile_width) as f32,
                    (j * ctx.atlas_scheme.tile_height) as f32
                );
            ctx.scene_compositor.borrow_mut().enqueue(RenderLayer::TileMap, draw_command);
        }
    }
}
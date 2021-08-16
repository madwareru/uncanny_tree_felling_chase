use crate::core_subsystems::types::GlobalContext;

pub fn is_half_tile_passable(ctx: &GlobalContext, x: usize, y: usize) -> bool {
    let x = x * (ctx.atlas_definition.tile_width / 2); // remapping coords just like when drawing
    let y = y * (ctx.atlas_definition.tile_height / 2);

    let x = x + ctx.atlas_definition.tile_width / 4;  // shift coords so they target
    let y = y + ctx.atlas_definition.tile_height / 4; // centers of half-tiles

    let idx = y * ctx.passability_map_stride + x; // get our ID finally

    ctx.passability_map.borrow()[idx] == 0xFF
}
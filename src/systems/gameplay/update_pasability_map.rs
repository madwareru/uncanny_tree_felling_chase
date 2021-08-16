use crate::core_subsystems::types::GlobalContext;

pub fn system(ctx: &GlobalContext) {
    ctx.passability_map.borrow_mut().fill(0x00);

    for j in 0..ctx.tilemap.borrow().h {
        for i in 0..ctx.tilemap.borrow().w {
            let idx = j * ctx.tilemap.borrow().w + i;
            let idx = ctx.tilemap.borrow().map_data[idx];
            let src_subrect = ctx.tilemap.borrow().tiles[idx];

            let j = j * ctx.atlas_definition.tile_height;
            let i = i * ctx.atlas_definition.tile_width;

            let mut dst_offset = j * ctx.passability_map_stride + i;
            let mut src_offset = src_subrect.y * ctx.passability_atlas_width + src_subrect.x;
            let span_length = src_subrect.width;

            let mut borrowed = ctx.passability_map.borrow_mut();

            for _ in 0..src_subrect.height {
                let zipped =
                    (&mut borrowed[dst_offset..dst_offset+span_length]).iter_mut()
                    .zip(&ctx.passability_atlas[src_offset..src_offset+span_length]);

                for (dest, source) in zipped { *dest = *source; }

                src_offset += ctx.passability_atlas_width;
                dst_offset += ctx.passability_map_stride;
            }
        }
    }
}
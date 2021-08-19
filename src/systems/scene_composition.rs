use crate::core_subsystems::types::GlobalContext;

pub fn system(ctx: &GlobalContext) {
    ctx.scene_compositor.borrow_mut().flush_drawing_queue(
        ctx.atlas_scheme.tile_width * ctx.tilemap.borrow().w,
        ctx.atlas_scheme.tile_height * ctx.tilemap.borrow().h,
        ctx.draw_scale
    );
}
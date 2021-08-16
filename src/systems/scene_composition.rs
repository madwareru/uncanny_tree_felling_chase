use crate::core_subsystems::rendering::SceneCompositor;
use crate::core_subsystems::types::GlobalContext;

pub fn system(ctx: &GlobalContext) {
    ctx.scene_compositor.borrow_mut().flush_drawing_queue(
        ctx.atlas_definition.tile_width * ctx.tilemap.w,
        ctx.atlas_definition.tile_height * ctx.tilemap.h,
        ctx.draw_scale
    );
}
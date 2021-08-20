use crate::core_subsystems::types::GlobalContext;
use crate::components::{FadeAroundMouseTag, Glyph, UiRect};

pub fn system(ctx: &GlobalContext) {
    for (_, (_, ui_rect, glyph)) in ctx.world.borrow_mut().query_mut::<(
        &FadeAroundMouseTag,
        &UiRect,
        &mut Glyph,
    )>() {
        let glyph_center_x = (ui_rect.top_left.0 + ui_rect.bottom_right.0) / 2;
        let glyph_center_y = (ui_rect.top_left.1 + ui_rect.bottom_right.1) / 2;
        let (peek_x, peek_y) = crate::core_subsystems::peek_utils::peek_tile(ctx);
        let dx = (peek_x - glyph_center_x) as f32;
        let dy = (peek_y - glyph_center_y) as f32;
        let distance = (dx*dx + dy*dy).sqrt();
        glyph.transparency = (distance / 10.0).clamp(0.1, 1.0);
    }
}
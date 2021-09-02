use crate::core_subsystems::types::GlobalContext;
use crate::components::{VisualLookDirection, Orc, Position, Animator};
use crate::core_subsystems::animation_configuration::{acquire_orc_animation_info, LookFlipsSpriteHorizontally};
use macro_tiler::atlas::draw_command::draw_command_builder;
use macro_tiler::atlas::rect_handle::{Having, DrawPivot, DrawFlip, DrawSizeOverride};
use crate::core_subsystems::rendering::RenderLayer;

pub fn system(ctx: &GlobalContext) {
    for (_, (_, position, animator, direction)) in ctx.world.borrow().query::<(
        &Orc,
        &Position,
        &Animator,
        &VisualLookDirection
    )>().iter() {
        let (flip_info, frames) = acquire_orc_animation_info(
            &animator.animation,
            &animator.state,
            &direction,
        );
        let frame = frames[animator.current_frame];
        let handle = ctx.orc_atlas.acquire_handle(&frame).unwrap();
        ctx.scene_compositor.borrow_mut().enqueue(
            RenderLayer::MapObjects,
            draw_command_builder()
                .having(DrawPivot::Relative([0.5, 1.0].into()))
                .having(DrawSizeOverride::ScaledUniform(1.2))
                .having(
                    match flip_info {
                        LookFlipsSpriteHorizontally::No => {
                            DrawFlip::None
                        }
                        LookFlipsSpriteHorizontally::Yes => {
                            DrawFlip::FlipX
                        }
                    })
                .build(
                    handle,
                    position.x as f32,
                    position.y as f32,
                ),
        );
    }
}
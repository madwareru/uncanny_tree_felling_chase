use macroquad::miniquad::Context as QuadCtx;
use macroquad::prelude::*;

fn get_context_mut<'a>() -> &'a mut QuadCtx {
    let InternalGlContext {
        quad_context: ctx, ..
    } = unsafe { get_internal_gl() };
    ctx
}

pub fn with_context<TResult>(action: impl FnOnce(&QuadCtx) -> TResult) -> TResult {
    let ctx = get_context_mut();
    action(ctx)
}

pub fn with_context_mut<TResult>(mut action: impl FnMut(&mut QuadCtx) -> TResult) -> TResult {
    let ctx = get_context_mut();
    action(ctx)
}
use crate::core_subsystems::types::GlobalContext;
use crate::components::SignalTag;

pub fn system(ctx: &GlobalContext) {
    for (entity, _) in ctx.world.borrow().query::<(&SignalTag,)>().iter() {
        ctx.enqueue_to_remove(entity)
    }
}
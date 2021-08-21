use crate::core_subsystems::types::GlobalContext;
use crate::components::Orc;

pub fn system(ctx: &GlobalContext) {
    for (entity, (_,)) in ctx.world.borrow().query::<(&Orc, )>().iter() { ctx.enqueue_to_remove(entity); }
}
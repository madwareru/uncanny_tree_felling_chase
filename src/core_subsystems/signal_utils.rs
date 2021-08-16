use crate::components::SignalTag;
use crate::core_subsystems::types::GlobalContext;

pub fn check_signal<TSignal>(ctx: &GlobalContext) -> Option<TSignal>
where TSignal: 'static + Copy + Clone + Send + Sync
{
    ctx.world.borrow()
        .query::<(&SignalTag, &TSignal)>()
        .iter()
        .next()
        .and_then(|(_,(_,&signal))| Some(signal))
}
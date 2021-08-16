use crate::core_subsystems::types::{MenuScreen, Fraction};
use crate::core_subsystems::atlas_serialization::SubRect;
use std::marker::PhantomData;

pub struct MenuBackgroundTag;

#[derive(Copy, Clone)]
pub struct UiRect {
    pub top_left: (i32, i32),
    pub bottom_right: (i32, i32),
}

#[derive(Copy, Clone)]
pub struct MenuScreenElement {
    pub menu_screen: MenuScreen
}

#[derive(Copy, Clone)]
pub struct SignalButton<TSignal: Copy + Clone> {
    pub signal_to_send: TSignal,
    pub glyph_sub_rect: SubRect
}

#[derive(Copy, Clone)]
pub struct ExitGameSignal;

#[derive(Copy, Clone)]
pub struct PlayGameSignal;

#[derive(Copy, Clone)]
pub struct ChoosePlayerFractionSignal {
    fraction: Fraction
}

#[derive(Copy, Clone)]
pub struct ChooseUnitTypeDuringLanding {
    is_big: bool
}

#[derive(Copy, Clone)]
pub struct GoToMainMenuSignal;

#[derive(Copy, Clone)]
pub struct PauseGameSignal;

#[derive(Copy, Clone)]
pub struct UnpauseGameSignal;

#[derive(Copy, Clone)]
pub struct ReplayGameSignal;
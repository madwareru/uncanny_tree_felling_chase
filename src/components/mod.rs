use crate::core_subsystems::types::{MenuScreen, Fraction};
use crate::core_subsystems::atlas_serialization::SubRect;
use std::marker::PhantomData;

pub struct MenuBackgroundTag;
pub struct SignalTag;

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
pub enum SignalCommand {
    ExitGame(ExitGameSignal),
    PlayGame(PlayGameSignal),
    ChoosePlayerFraction(ChoosePlayerFractionSignal),
    ChooseUnitTypeDuringLanding(ChooseUnitTypeDuringLanding),
    GoToMainMenu(GoToMainMenuSignal),
    PauseGame(PauseGameSignal),
    UnpauseGame(UnpauseGameSignal),
    ReplayGame(ReplayGameSignal),
}

#[derive(Copy, Clone)]
pub struct SignalButton<TSignal: Copy + Clone> {
    pub signal_to_send: TSignal,
    pub glyph_sub_rect: SubRect
}

#[derive(Copy, Clone)]
pub struct ExitGameSignal;
impl Into<SignalCommand> for ExitGameSignal {
    fn into(self) -> SignalCommand {
        SignalCommand::ExitGame(self)
    }
}

#[derive(Copy, Clone)]
pub struct PlayGameSignal;
impl Into<SignalCommand> for PlayGameSignal {
    fn into(self) -> SignalCommand {
        SignalCommand::PlayGame(self)
    }
}

#[derive(Copy, Clone)]
pub struct ChoosePlayerFractionSignal {
    fraction: Fraction
}
impl Into<SignalCommand> for ChoosePlayerFractionSignal {
    fn into(self) -> SignalCommand {
        SignalCommand::ChoosePlayerFraction(self)
    }
}

#[derive(Copy, Clone)]
pub struct ChooseUnitTypeDuringLanding {
    is_big: bool
}
impl Into<SignalCommand> for ChooseUnitTypeDuringLanding {
    fn into(self) -> SignalCommand {
        SignalCommand::ChooseUnitTypeDuringLanding(self)
    }
}

#[derive(Copy, Clone)]
pub struct GoToMainMenuSignal;
impl Into<SignalCommand> for GoToMainMenuSignal {
    fn into(self) -> SignalCommand {
        SignalCommand::GoToMainMenu(self)
    }
}

#[derive(Copy, Clone)]
pub struct PauseGameSignal;
impl Into<SignalCommand> for PauseGameSignal {
    fn into(self) -> SignalCommand {
        SignalCommand::PauseGame(self)
    }
}

#[derive(Copy, Clone)]
pub struct UnpauseGameSignal;
impl Into<SignalCommand> for UnpauseGameSignal {
    fn into(self) -> SignalCommand {
        SignalCommand::UnpauseGame(self)
    }
}

#[derive(Copy, Clone)]
pub struct ReplayGameSignal;
impl Into<SignalCommand> for ReplayGameSignal {
    fn into(self) -> SignalCommand {
        SignalCommand::ReplayGame(self)
    }
}
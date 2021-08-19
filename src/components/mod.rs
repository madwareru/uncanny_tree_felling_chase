use crate::core_subsystems::types::{MenuScreen, Fraction};
use macro_tiler::atlas::rect_handle::AtlasRectHandle;

pub struct MenuBackgroundTag;
pub struct SignalTag;
pub struct SelectionTag;
pub struct BudgetDigitTag;
pub struct BudgetTitleTag;
pub struct ToggleButtonTag;

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
    FinishPlayerLanding(FinishPlayerLandingSignal),
    GoToMainMenu(GoToMainMenuSignal),
    PauseGame(PauseGameSignal),
    UnpauseGame(UnpauseGameSignal),
    ReplayGame(ReplayGameSignal),
}

#[derive(Copy, Clone)]
pub struct SignalButton<TSignal: Copy + Clone> {
    pub signal_to_send: TSignal,
    pub rect_handle: AtlasRectHandle
}

#[derive(Copy, Clone)]
pub struct Glyph {
    pub rect_handle: AtlasRectHandle
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
    pub fraction: Fraction
}
impl Into<SignalCommand> for ChoosePlayerFractionSignal {
    fn into(self) -> SignalCommand {
        SignalCommand::ChoosePlayerFraction(self)
    }
}

#[derive(Copy, Clone)]
pub struct ChooseUnitTypeDuringLanding {
    pub new_minion_is_big: bool
}
impl Into<SignalCommand> for ChooseUnitTypeDuringLanding {
    fn into(self) -> SignalCommand {
        SignalCommand::ChooseUnitTypeDuringLanding(self)
    }
}

#[derive(Copy, Clone)]
pub struct FinishPlayerLandingSignal;
impl Into<SignalCommand> for FinishPlayerLandingSignal {
    fn into(self) -> SignalCommand {
        SignalCommand::FinishPlayerLanding(self)
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

#[derive(Copy, Clone)]
pub struct NumberTag(pub u32);
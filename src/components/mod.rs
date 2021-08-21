use crate::core_subsystems::types::{MenuScreen, Fraction};
use macro_tiler::atlas::rect_handle::AtlasRectHandle;
use crate::core_subsystems::animation_configuration::{AnimationState, OrcAnimation};
use crate::core_subsystems::atlas_serialization::TreeType;

pub struct MenuBackgroundTag;
pub struct SignalTag;
pub struct SelectionTag;
pub struct BudgetDigitTag;
pub struct BudgetTitleTag;
pub struct ToggleButtonTag;
pub struct FadeAroundMouseTag;

#[derive(Copy, Clone)]
pub struct Orc{
    pub is_huge: bool,
    pub fraction: Fraction
}

#[derive(Copy, Clone)]
pub struct Animator {
    pub animation: OrcAnimation,
    pub state: AnimationState,
    pub current_frame: usize,
    pub current_ticks: usize,
}

#[derive(Copy, Clone)]
pub struct Direction {
    pub dir_x: i32,
    pub dir_y: i32,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub enum VisualLookDirection {
    SouthEast = 0,
    SouthWest = 1,
    NorthEast = 2,
    NorthWest = 3
}

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
    ClearAllUnits(ClearAllUnitsSignal)
}

#[derive(Copy, Clone)]
pub struct SignalButton<TSignal: Copy + Clone> {
    pub signal_to_send: TSignal,
    pub rect_handle: AtlasRectHandle
}

#[derive(Copy, Clone)]
pub struct Glyph {
    pub rect_handle: AtlasRectHandle,
    pub transparency: f32
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
pub struct ClearAllUnitsSignal;
impl Into<SignalCommand> for ClearAllUnitsSignal {
    fn into(self) -> SignalCommand {
        SignalCommand::ClearAllUnits(self)
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

#[derive(Copy, Clone)]
pub struct SpareBudgetSignal(pub u32);

#[derive(Copy, Clone)]
pub struct GrowBudgetSignal(pub u32);
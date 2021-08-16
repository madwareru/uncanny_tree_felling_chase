use std::sync::Arc;

use macroquad::prelude::*;

use crate::core_subsystems::atlas_serialization::{AtlasDefinition};
use crate::core_subsystems::forest::Forest;
use crate::core_subsystems::rendering::SceneCompositor;
use crate::core_subsystems::tilemap::Tilemap;
use std::cell::RefCell;

pub type CustomBitSet = [u8; 32];

pub struct GlobalContext {
    pub atlas_definition: Arc<AtlasDefinition>,
    pub atlas_texture: Texture2D,
    pub forest: Forest,
    pub tilemap: Tilemap,
    pub ui_atlas_texture: Texture2D,
    pub draw_scale: f32,
    pub game_state: RefCell<GameState>,
    pub scene_compositor: RefCell<SceneCompositor>,
    pub world: RefCell<hecs::World>
}

#[derive(Copy, Clone, PartialEq)]
pub enum Fraction {
    Red,
    Blue,
}

#[derive(Copy, Clone, PartialEq)]
pub enum GameState {
    MainMenu,
    FractionChoice,
    Battle {
        fraction: Fraction,
        internal_state: BattleState
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum BattleState {
    MapGeneration,
    EnemyLanding,
    PlayerLanding,
    BattlePause,
    Defeat,
    Victory,
    Simulation {
        red_score: u16,
        blue_score: u16
    },
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MenuScreen {
    MainMenu,
    FractionChoice,
    PlayerLanding,
    BattleSimulation,
    BattlePause,
    Defeat,
    Victory
}

impl GameState {
    pub fn get_menu_screen(&self) -> Option<MenuScreen> {
        match self {
            GameState::MainMenu => Some(MenuScreen::MainMenu),
            GameState::FractionChoice => Some(MenuScreen::FractionChoice),
            GameState::Battle { internal_state, .. } => internal_state.get_menu_screen()
        }
    }
}

impl BattleState {
    pub fn get_menu_screen(&self) -> Option<MenuScreen> {
        match self {
            BattleState::MapGeneration => None,
            BattleState::EnemyLanding => None,
            BattleState::PlayerLanding => Some(MenuScreen::PlayerLanding),
            BattleState::BattlePause => Some(MenuScreen::BattlePause),
            BattleState::Defeat => Some(MenuScreen::Defeat),
            BattleState::Victory => Some(MenuScreen::Victory),
            BattleState::Simulation { .. } => Some(MenuScreen::BattleSimulation)
        }
    }
}
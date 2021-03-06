use std::sync::Arc;

use macroquad::prelude::*;

use crate::core_subsystems::atlas_serialization::{AtlasScheme, MainTile, UiTile, OrcSprite};
use crate::core_subsystems::forest::Forest;
use crate::core_subsystems::rendering::SceneCompositor;
use crate::core_subsystems::tilemap::Tilemap;
use std::cell::RefCell;
use std::collections::VecDeque;
use crate::components::{SignalCommand, SignalTag};
use crate::core_subsystems::units_serialization::UnitsConfig;
use crate::core_subsystems::player_landing_info::MapFieldOccupationData;

pub type CustomBitSet = [u8; 27];

pub struct GlobalContext {
    pub atlas_scheme: Arc<AtlasScheme>,
    pub units_config: Arc<UnitsConfig>,

    pub main_atlas_definition: Arc<macro_tiler::atlas::AtlasDefinition<MainTile>>,
    pub ui_atlas_definition: Arc<macro_tiler::atlas::AtlasDefinition<UiTile>>,

    pub main_atlas: Arc<macro_tiler::atlas::Atlas<MainTile>>,
    pub ui_atlas: Arc<macro_tiler::atlas::Atlas<UiTile>>,
    pub orc_atlas: Arc<macro_tiler::atlas::Atlas<OrcSprite>>,

    pub game_state: RefCell<GameState>,
    pub landing_occupation_data: RefCell<MapFieldOccupationData>,
    pub scene_compositor: RefCell<SceneCompositor>,
    pub world: RefCell<hecs::World>,
    pub signal_command_buffer: RefCell<VecDeque<SignalCommand>>,
    pub entity_purgatory: RefCell<VecDeque<hecs::Entity>>,
    pub forest: RefCell<Forest>,
    pub tilemap: RefCell<Tilemap>,
    pub passability_map: RefCell<Vec<u8>>,

    pub draw_scale: f32,
    pub passability_atlas_width: usize,
    pub passability_atlas_height: usize,
    pub passability_atlas: Vec<u8>,
    pub passability_map_stride: usize,
}

impl GlobalContext {
    pub(crate) fn enqueue_to_remove(&self, entity: hecs::Entity) {
        self.entity_purgatory.borrow_mut().push_back(entity);
    }

    pub fn flush_command_queues(&self) {
        let mut signal_commands = self.signal_command_buffer.borrow_mut();
        while !signal_commands.is_empty() {
            let next_command = signal_commands.pop_front().unwrap();
            match next_command {
                SignalCommand::ExitGame(signal) => self.world.borrow_mut().spawn((
                    SignalTag,
                    signal
                )),
                SignalCommand::PlayGame(signal) => self.world.borrow_mut().spawn((
                    SignalTag,
                    signal
                )),
                SignalCommand::ChoosePlayerFraction(signal) => self.world.borrow_mut().spawn((
                    SignalTag,
                    signal
                )),
                SignalCommand::ChooseUnitTypeDuringLanding(signal) => self.world.borrow_mut().spawn((
                    SignalTag,
                    signal
                )),
                SignalCommand::GoToMainMenu(signal) => self.world.borrow_mut().spawn((
                    SignalTag,
                    signal
                )),
                SignalCommand::PauseGame(signal) => self.world.borrow_mut().spawn((
                    SignalTag,
                    signal
                )),
                SignalCommand::UnpauseGame(signal) => self.world.borrow_mut().spawn((
                    SignalTag,
                    signal
                )),
                SignalCommand::ReplayGame(signal) => self.world.borrow_mut().spawn((
                    SignalTag,
                    signal
                )),
                SignalCommand::FinishPlayerLanding(signal) => self.world.borrow_mut().spawn((
                    SignalTag,
                    signal
                )),
                SignalCommand::ClearAllUnits(signal) => self.world.borrow_mut().spawn((
                    SignalTag,
                    signal
                )),
            };
        }

        let mut dead_entities = self.entity_purgatory.borrow_mut();
        while !dead_entities.is_empty() {
            let next_entity = dead_entities.pop_front().unwrap();
            self.world.borrow_mut().despawn(next_entity).unwrap();
        }
    }
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
    Defeat,
    Victory,
    PreparePlayerLanding,
    PlayerLanding {
        budget: u32,
        current_minion_is_big: bool
    },
    EnemyLanding,
    Simulation {
        is_paused: bool,
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
            BattleState::PlayerLanding { .. } => Some(MenuScreen::PlayerLanding),
            BattleState::Defeat => Some(MenuScreen::Defeat),
            BattleState::Victory => Some(MenuScreen::Victory),
            BattleState::Simulation { is_paused, .. } => if *is_paused {
                Some(MenuScreen::BattlePause)
            } else {
                Some(MenuScreen::BattleSimulation)
            },
            BattleState::PreparePlayerLanding => None
        }
    }
}
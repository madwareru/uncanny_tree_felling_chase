use serde::Deserialize;
use std::collections::HashMap;
use crate::core_subsystems::atlas_serialization::TreeType;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
pub enum OrcType {
    SmallOrc,
    HugeOrc,
}

#[derive(Copy, Clone, PartialEq, Debug, Deserialize)]
pub enum OrcAbility {
    Repulsion
}

#[derive(Copy, Clone, PartialEq, Debug, Deserialize)]
pub struct  OrcConfig {
    pub price: u32,
    pub movement_speed: i32,
    pub turn_speed: i32,
    pub damage: i32,
    pub ability: Option<OrcAbility>
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct UnitsConfig {
    pub tree_health: HashMap<TreeType, i32>,
    pub orcs: HashMap<OrcType, OrcConfig>
}
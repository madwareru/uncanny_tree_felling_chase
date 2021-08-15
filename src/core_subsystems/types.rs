use std::sync::Arc;

use macroquad::prelude::*;

use crate::core_subsystems::atlas_serialization::{AtlasDefinition};
use crate::core_subsystems::forest::Forest;
use crate::core_subsystems::rendering::SceneCompositor;
use crate::core_subsystems::tilemap::Tilemap;

pub type CustomBitSet = [u8; 32];

pub struct GlobalStorage {
    pub atlas_definition: Arc<AtlasDefinition>,
    pub atlas_texture: Texture2D,
    pub forest: Forest,
    pub tilemap: Tilemap,
    pub scene_compositor: SceneCompositor,
    pub ui_atlas_texture: Texture2D,
    pub draw_scale: f32
}
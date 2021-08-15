use crate::{
    tilemap::Tilemap,
    forest::Forest,
    rendering::SceneCompositor,
    atlas_serialization::AtlasDefinition
};
use std::sync::Arc;
use macroquad::prelude::*;

pub type CustomBitSet = [u8; 32];

pub struct GlobalStorage {
    pub atlas_definition: Arc<AtlasDefinition>,
    pub atlas_texture: Texture2D,
    pub forest: Forest,
    pub tilemap: Tilemap,
    pub scene_compositor: SceneCompositor
}
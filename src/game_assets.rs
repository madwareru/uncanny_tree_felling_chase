use {
    macroquad::prelude::*,
    macroquad::miniquad::{TextureFormat, TextureWrap},
    ron::de::from_reader,
    std::sync::Arc
};
use crate::core_subsystems::atlas_serialization::{AtlasScheme, MainTile, UiTile, OrcSprite};
use crate::core_subsystems::units_serialization::UnitsConfig;

const ATLAS_SCHEME_BYTES: &[u8] = include_bytes!("../assets/atlas_scheme.ron");
const MAIN_ATLAS_DEFINITION_BYTES: &[u8] = include_bytes!("../assets/main_atlas.ron");
const UI_ATLAS_DEFINITION_BYTES: &[u8] = include_bytes!("../assets/ui_atlas.ron");
const ORC_ATLAS_DEFINITION_BYTES: &[u8] = include_bytes!("../assets/unit_sprites.ron");
const UNITS_CONFIG_BYTES: &[u8] = include_bytes!("../assets/units_config.ron");

const MAIN_ATLAS_BYTES: &[u8] = include_bytes!("../assets/main_atlas.png");
const UI_ATLAS_BYTES: &[u8] = include_bytes!("../assets/ui_atlas.png");
const ORC_ATLAS_BYTES: &[u8] = include_bytes!("../assets/unit_sprites.png");
const PASSABILITY_MAP_BYTES: &[u8] = include_bytes!("../assets/passability_map.png");

pub struct GameAssets {
    pub main_atlas_definition: Arc<macro_tiler::atlas::AtlasDefinition<MainTile>>,
    pub ui_atlas_definition: Arc<macro_tiler::atlas::AtlasDefinition<UiTile>>,

    pub main_atlas: Arc<macro_tiler::atlas::Atlas<MainTile>>,
    pub ui_atlas: Arc<macro_tiler::atlas::Atlas<UiTile>>,
    pub orc_atlas: Arc<macro_tiler::atlas::Atlas<OrcSprite>>,

    pub units_config: Arc<UnitsConfig>,
    pub atlas_definition: Arc<AtlasScheme>,
    pub passability_atlas_width: usize,
    pub passability_atlas_height: usize,
    pub passability_atlas: Vec<u8>
}

impl GameAssets {
    pub(crate) fn load() -> Self {
        let main_atlas_definition = Arc::new(from_reader(MAIN_ATLAS_DEFINITION_BYTES).unwrap());
        let main_atlas = Arc::new(macro_tiler::atlas::Atlas::load(
            &main_atlas_definition,
            MAIN_ATLAS_BYTES,
            3,
            TextureFormat::RGBA8,
            FilterMode::Linear,
            TextureWrap::Clamp
        ));

        let ui_atlas_definition = Arc::new(from_reader(UI_ATLAS_DEFINITION_BYTES).unwrap());
        let ui_atlas = Arc::new(macro_tiler::atlas::Atlas::load(
            &ui_atlas_definition,
            UI_ATLAS_BYTES,
            3,
            TextureFormat::RGBA8,
            FilterMode::Linear,
            TextureWrap::Clamp
        ));

        let orc_atlas_definition = Arc::new(from_reader(ORC_ATLAS_DEFINITION_BYTES).unwrap());
        let orc_atlas = Arc::new(macro_tiler::atlas::Atlas::load(
            &orc_atlas_definition,
            ORC_ATLAS_BYTES,
            3,
            TextureFormat::RGBA8,
            FilterMode::Linear,
            TextureWrap::Clamp
        ));

        let (
            passability_atlas_width,
            passability_atlas_height,
            passability_atlas
        ) = GameAssets::load_passability_atlas(PASSABILITY_MAP_BYTES);

        Self {
            main_atlas_definition,
            ui_atlas_definition,

            main_atlas,
            ui_atlas,
            orc_atlas,

            passability_atlas_width,
            passability_atlas_height,
            passability_atlas,
            atlas_definition: Arc::new(from_reader(ATLAS_SCHEME_BYTES).unwrap()),
            units_config: Arc::new(from_reader(UNITS_CONFIG_BYTES).unwrap())
        }
    }

    fn load_passability_atlas(bytes: &[u8]) -> (usize, usize, Vec<u8>) {
        let img = image::load_from_memory(bytes)
            .unwrap_or_else(|e| panic!("{}", e))
            .to_rgba8();

        (
            img.width() as usize, img.height() as usize,
            img.into_raw().iter().skip(1)// check second component only
                .step_by(4).map(|&it| if it > 0x88 { 0xFF } else { 0x00 }).collect::<Vec<_>>()
        )
    }
}
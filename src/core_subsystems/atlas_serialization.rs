use serde::Deserialize;

#[derive(Copy, Clone, PartialEq, Debug, Deserialize)]
pub enum TerrainType {
    Land,
    GrassSharp,
    GrassRound,
    Water,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
pub enum TreeType {
    None,
    Pine,
    Oak,
    Bush,
    Stump,
}

#[derive(Copy, Clone, PartialEq, Debug, Deserialize)]
pub enum TileKind {
    Inner,
    Outer,
}

#[derive(Copy, Clone, PartialEq, Debug, Deserialize)]
pub enum NeighbourKind {
    WangCorners,
    RelOffset(i32),
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct NeighbourChooseStrategy {
    pub north: NeighbourKind,
    pub west: NeighbourKind,
    pub east: NeighbourKind,
    pub south: NeighbourKind,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct TileSidesPattern {
    pub north_west: TileKind,
    pub north_east: TileKind,
    pub south_west: TileKind,
    pub south_east: TileKind,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct TileSides {
    pub north_west: TerrainType,
    pub north_east: TerrainType,
    pub south_west: TerrainType,
    pub south_east: TerrainType,
}

#[derive(Copy, Clone, Deserialize)]
pub struct TerrainTilesConfig {
    pub offset: usize,
    pub outer_type: TerrainType,
    pub inner_type: TerrainType,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Deserialize, Hash, Eq, PartialEq)]
pub enum OrcSprite {
    RedOrcSmall_Idle,
    RedOrcSmall_Hit_0,
    RedOrcSmall_Hit_1,
    RedOrcSmall_Hit_2,
    RedOrcSmall_Hit_3,
    RedOrcSmall_Hit_4,
    RedOrcSmall_Hit_5,
    RedOrcSmall_Hit_6,

    RedOrcHuge_Idle,
    RedOrcHuge_Hit_0,
    RedOrcHuge_Hit_1,
    RedOrcHuge_Hit_2,
    RedOrcHuge_Hit_3,
    RedOrcHuge_Hit_4,
    RedOrcHuge_Hit_5,
    RedOrcHuge_Hit_6,

    RedOrcSmall_IdleBack,
    RedOrcSmall_HitBack_0,
    RedOrcSmall_HitBack_1,
    RedOrcSmall_HitBack_2,
    RedOrcSmall_HitBack_3,
    RedOrcSmall_HitBack_4,
    RedOrcSmall_HitBack_5,
    RedOrcSmall_HitBack_6,

    RedOrcHuge_IdleBack,
    RedOrcHuge_HitBack_0,
    RedOrcHuge_HitBack_1,
    RedOrcHuge_HitBack_2,
    RedOrcHuge_HitBack_3,
    RedOrcHuge_HitBack_4,
    RedOrcHuge_HitBack_5,
    RedOrcHuge_HitBack_6,

    BlueOrcSmall_Idle,
    BlueOrcSmall_Hit_0,
    BlueOrcSmall_Hit_1,
    BlueOrcSmall_Hit_2,
    BlueOrcSmall_Hit_3,
    BlueOrcSmall_Hit_4,
    BlueOrcSmall_Hit_5,
    BlueOrcSmall_Hit_6,

    BlueOrcHuge_Idle,
    BlueOrcHuge_Hit_0,
    BlueOrcHuge_Hit_1,
    BlueOrcHuge_Hit_2,
    BlueOrcHuge_Hit_3,
    BlueOrcHuge_Hit_4,
    BlueOrcHuge_Hit_5,
    BlueOrcHuge_Hit_6,

    BlueOrcSmall_IdleBack,
    BlueOrcSmall_HitBack_0,
    BlueOrcSmall_HitBack_1,
    BlueOrcSmall_HitBack_2,
    BlueOrcSmall_HitBack_3,
    BlueOrcSmall_HitBack_4,
    BlueOrcSmall_HitBack_5,
    BlueOrcSmall_HitBack_6,

    BlueOrcHuge_IdleBack,
    BlueOrcHuge_HitBack_0,
    BlueOrcHuge_HitBack_1,
    BlueOrcHuge_HitBack_2,
    BlueOrcHuge_HitBack_3,
    BlueOrcHuge_HitBack_4,
    BlueOrcHuge_HitBack_5,
    BlueOrcHuge_HitBack_6,
}

macro_rules! sequence_const{
    { pub const $name:ident: &[$type:ident] = $id:ident ++ [$($lit:literal),*] } => {
        paste::paste! {
            pub const $name: &[$type] = &[
                $($type :: [<$id $lit>]),*
            ];
        }
    }
}

sequence_const! {
    pub const RED_DIGIT_GLYPH_TILES: &[UiTile] = RedDigitGlyph_ ++ [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
}

sequence_const! {
    pub const BLUE_DIGIT_GLYPH_TILES: &[UiTile] = BlueDigitGlyph_ ++ [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Deserialize, Hash, Eq, PartialEq)]
pub enum UiTile {
    GenerateInProgress,
    PlayButtonLabel,
    ExitButtonLabel,
    RedFractionButtonLabel,
    BackButtonLabel,
    ReadyButtonLabel,
    BlueFractionButtonLabel,
    ReplayButtonLabel,
    MainMenuButtonLabel,
    GameTitle,
    LandingTitle,
    ChooseYourSideTitle,
    DefeatTitle,
    VictoryTitle,
    RedBudgetTitle,
    BlueBudgetTitle,

    RedDigitGlyph_0,
    RedDigitGlyph_1,
    RedDigitGlyph_2,
    RedDigitGlyph_3,
    RedDigitGlyph_4,
    RedDigitGlyph_5,
    RedDigitGlyph_6,
    RedDigitGlyph_7,
    RedDigitGlyph_8,
    RedDigitGlyph_9,

    BlueDigitGlyph_0,
    BlueDigitGlyph_1,
    BlueDigitGlyph_2,
    BlueDigitGlyph_3,
    BlueDigitGlyph_4,
    BlueDigitGlyph_5,
    BlueDigitGlyph_6,
    BlueDigitGlyph_7,
    BlueDigitGlyph_8,
    BlueDigitGlyph_9,

    UiBorders3x3_0,
    UiBorders3x3_1,
    UiBorders3x3_2,
    UiBorders3x3_3,
    UiBorders3x3_4,
    UiBorders3x3_5,
    UiBorders3x3_6,
    UiBorders3x3_7,
    UiBorders3x3_8,

    UiBackgroundBox3x3_0,
    UiBackgroundBox3x3_1,
    UiBackgroundBox3x3_2,
    UiBackgroundBox3x3_3,
    UiBackgroundBox3x3_4,
    UiBackgroundBox3x3_5,
    UiBackgroundBox3x3_6,
    UiBackgroundBox3x3_7,
    UiBackgroundBox3x3_8,

    UiSelection3x3_0,
    UiSelection3x3_1,
    UiSelection3x3_2,
    UiSelection3x3_3,
    UiSelection3x3_4,
    UiSelection3x3_5,
    UiSelection3x3_6,
    UiSelection3x3_7,
    UiSelection3x3_8,

    ButtonIdle3x3_0,
    ButtonIdle3x3_1,
    ButtonIdle3x3_2,
    ButtonIdle3x3_3,
    ButtonIdle3x3_4,
    ButtonIdle3x3_5,
    ButtonIdle3x3_6,
    ButtonIdle3x3_7,
    ButtonIdle3x3_8,

    ButtonClicked3x3_0,
    ButtonClicked3x3_1,
    ButtonClicked3x3_2,
    ButtonClicked3x3_3,
    ButtonClicked3x3_4,
    ButtonClicked3x3_5,
    ButtonClicked3x3_6,
    ButtonClicked3x3_7,
    ButtonClicked3x3_8,

    ButtonHover3x3_0,
    ButtonHover3x3_1,
    ButtonHover3x3_2,
    ButtonHover3x3_3,
    ButtonHover3x3_4,
    ButtonHover3x3_5,
    ButtonHover3x3_6,
    ButtonHover3x3_7,
    ButtonHover3x3_8,

    SmallAxeIcon,
    HugeAxeIcon,

    SelectedCell,
    BlockedCell,
    OkCell,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Deserialize, Hash, Eq, PartialEq)]
pub enum MainTile {
    Land_GrassSharp_0,
    Land_GrassSharp_1,
    Land_GrassSharp_2,
    Land_GrassSharp_3,
    Land_GrassSharp_4,
    Land_GrassSharp_5,
    Land_GrassSharp_6,
    Land_GrassSharp_7,
    Land_GrassSharp_8,
    Land_GrassSharp_9,
    Land_GrassSharp_10,
    Land_GrassSharp_11,
    Land_GrassSharp_12,
    Land_GrassSharp_13,
    Land_GrassSharp_14,
    Land_GrassSharp_15,
    Land_GrassSharp_16,
    Land_GrassSharp_17,
    Land_GrassSharp_18,
    Land_GrassSharp_19,
    Land_GrassSharp_20,
    Land_GrassSharp_21,
    Land_GrassSharp_22,
    Land_GrassSharp_23,
    Land_GrassSharp_24,
    Land_GrassSharp_25,
    Land_GrassSharp_26,
    Land_GrassSharp_27,
    Land_GrassSharp_28,
    Land_GrassSharp_29,
    Land_GrassSharp_30,
    Land_GrassSharp_31,
    Land_GrassSharp_32,
    Land_GrassSharp_33,
    Land_GrassSharp_34,
    Land_GrassSharp_35,
    Land_GrassSharp_36,
    Land_GrassSharp_37,
    Land_GrassSharp_38,
    Land_GrassSharp_39,
    Land_GrassSharp_40,
    Land_GrassSharp_41,
    Land_GrassSharp_42,
    Land_GrassSharp_43,
    Land_GrassSharp_44,
    Land_GrassSharp_45,
    Land_GrassSharp_46,
    Land_GrassSharp_47,

    GrassRound_GrassSharp_0,
    GrassRound_GrassSharp_1,
    GrassRound_GrassSharp_2,
    GrassRound_GrassSharp_3,
    GrassRound_GrassSharp_4,
    GrassRound_GrassSharp_5,
    GrassRound_GrassSharp_6,
    GrassRound_GrassSharp_7,
    GrassRound_GrassSharp_8,
    GrassRound_GrassSharp_9,
    GrassRound_GrassSharp_10,
    GrassRound_GrassSharp_11,
    GrassRound_GrassSharp_12,
    GrassRound_GrassSharp_13,
    GrassRound_GrassSharp_14,
    GrassRound_GrassSharp_15,
    GrassRound_GrassSharp_16,
    GrassRound_GrassSharp_17,
    GrassRound_GrassSharp_18,
    GrassRound_GrassSharp_19,
    GrassRound_GrassSharp_20,
    GrassRound_GrassSharp_21,
    GrassRound_GrassSharp_22,
    GrassRound_GrassSharp_23,
    GrassRound_GrassSharp_24,
    GrassRound_GrassSharp_25,
    GrassRound_GrassSharp_26,
    GrassRound_GrassSharp_27,
    GrassRound_GrassSharp_28,
    GrassRound_GrassSharp_29,
    GrassRound_GrassSharp_30,
    GrassRound_GrassSharp_31,
    GrassRound_GrassSharp_32,
    GrassRound_GrassSharp_33,
    GrassRound_GrassSharp_34,
    GrassRound_GrassSharp_35,
    GrassRound_GrassSharp_36,
    GrassRound_GrassSharp_37,
    GrassRound_GrassSharp_38,
    GrassRound_GrassSharp_39,
    GrassRound_GrassSharp_40,
    GrassRound_GrassSharp_41,
    GrassRound_GrassSharp_42,
    GrassRound_GrassSharp_43,
    GrassRound_GrassSharp_44,
    GrassRound_GrassSharp_45,
    GrassRound_GrassSharp_46,
    GrassRound_GrassSharp_47,

    Land_GrassRound_0,
    Land_GrassRound_1,
    Land_GrassRound_2,
    Land_GrassRound_3,
    Land_GrassRound_4,
    Land_GrassRound_5,
    Land_GrassRound_6,
    Land_GrassRound_7,
    Land_GrassRound_8,
    Land_GrassRound_9,
    Land_GrassRound_10,
    Land_GrassRound_11,
    Land_GrassRound_12,
    Land_GrassRound_13,
    Land_GrassRound_14,
    Land_GrassRound_15,
    Land_GrassRound_16,
    Land_GrassRound_17,
    Land_GrassRound_18,
    Land_GrassRound_19,
    Land_GrassRound_20,
    Land_GrassRound_21,
    Land_GrassRound_22,
    Land_GrassRound_23,
    Land_GrassRound_24,
    Land_GrassRound_25,
    Land_GrassRound_26,
    Land_GrassRound_27,
    Land_GrassRound_28,
    Land_GrassRound_29,
    Land_GrassRound_30,
    Land_GrassRound_31,
    Land_GrassRound_32,
    Land_GrassRound_33,
    Land_GrassRound_34,
    Land_GrassRound_35,
    Land_GrassRound_36,
    Land_GrassRound_37,
    Land_GrassRound_38,
    Land_GrassRound_39,
    Land_GrassRound_40,
    Land_GrassRound_41,
    Land_GrassRound_42,
    Land_GrassRound_43,
    Land_GrassRound_44,
    Land_GrassRound_45,
    Land_GrassRound_46,
    Land_GrassRound_47,

    Land_Water_0,
    Land_Water_1,
    Land_Water_2,
    Land_Water_3,
    Land_Water_4,
    Land_Water_5,
    Land_Water_6,
    Land_Water_7,
    Land_Water_8,
    Land_Water_9,
    Land_Water_10,
    Land_Water_11,
    Land_Water_12,
    Land_Water_13,
    Land_Water_14,
    Land_Water_15,
    Land_Water_16,
    Land_Water_17,
    Land_Water_18,
    Land_Water_19,
    Land_Water_20,
    Land_Water_21,
    Land_Water_22,
    Land_Water_23,
    Land_Water_24,
    Land_Water_25,
    Land_Water_26,
    Land_Water_27,
    Land_Water_28,
    Land_Water_29,
    Land_Water_30,
    Land_Water_31,
    Land_Water_32,
    Land_Water_33,
    Land_Water_34,
    Land_Water_35,
    Land_Water_36,
    Land_Water_37,
    Land_Water_38,
    Land_Water_39,
    Land_Water_40,
    Land_Water_41,
    Land_Water_42,
    Land_Water_43,
    Land_Water_44,
    Land_Water_45,
    Land_Water_46,
    Land_Water_47,

    HorizontalBridge_0,
    HorizontalBridge_1,
    HorizontalBridge_2,
    HorizontalBridge_3,
    HorizontalBridge_4,
    HorizontalBridge_5,
    HorizontalBridge_6,
    HorizontalBridge_7,
    HorizontalBridge_8,

    VerticalBridge_0,
    VerticalBridge_1,
    VerticalBridge_2,
    VerticalBridge_3,
    VerticalBridge_4,
    VerticalBridge_5,
    VerticalBridge_6,
    VerticalBridge_7,
    VerticalBridge_8,

    Pine,
    Oak,
    Bush,
    Stump,
}

#[derive(Clone, Deserialize)]
pub struct AtlasScheme {
    pub tile_width: usize,
    pub tile_height: usize,

    pub reduced_wang_patterns: Vec<TileSidesPattern>,
    pub extended_set_1_patterns_north_west: Vec<TileSidesPattern>,
    pub extended_set_1_patterns_north_east: Vec<TileSidesPattern>,
    pub extended_set_1_patterns_south_west: Vec<TileSidesPattern>,
    pub extended_set_1_patterns_south_east: Vec<TileSidesPattern>,
    pub extended_set_2_patterns_north_west: Vec<TileSidesPattern>,
    pub extended_set_2_patterns_north_east: Vec<TileSidesPattern>,
    pub extended_set_2_patterns_south_west: Vec<TileSidesPattern>,
    pub extended_set_2_patterns_south_east: Vec<TileSidesPattern>,

    pub vertical_bridge_sides: Vec<TileSides>,
    pub horizontal_bridge_sides: Vec<TileSides>,

    pub reduced_wang_neighbour_strategy: Vec<NeighbourChooseStrategy>,
    pub neighbour_strategy_2_x_2: Vec<NeighbourChooseStrategy>,
    pub neighbour_strategy_3_x_3: Vec<NeighbourChooseStrategy>,

    pub terrain_tile_configs: Vec<TerrainTilesConfig>,
}
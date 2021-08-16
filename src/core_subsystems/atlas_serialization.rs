use serde::Deserialize;
use std::collections::HashMap;

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
    pub x_offset: i32,
    pub y_offset: i32,
    pub outer_type: TerrainType,
    pub inner_type: TerrainType,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct SubRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Deserialize)]
pub struct AtlasDefinition {
    pub tile_width: usize,
    pub tile_height: usize,

    pub tree_sub_rects: HashMap<TreeType, SubRect>,

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

    pub play_button_subrect: SubRect,
    pub exit_button_subrect: SubRect,
    pub red_button_subrect: SubRect,
    pub back_button_subrect: SubRect,
    pub ready_button_subrect: SubRect,
    pub blue_button_subrect: SubRect,
    pub replay_button_subrect: SubRect,
    pub main_menu_button_subrect: SubRect,
    pub game_title_subrect: SubRect,
    pub choose_your_side_title_subrect: SubRect,
    pub victory_title_subrect: SubRect,
    pub defeat_title_subrect: SubRect,

    pub ui_borders_3x3_0: SubRect,
    pub ui_borders_3x3_1: SubRect,
    pub ui_borders_3x3_2: SubRect,
    pub ui_borders_3x3_3: SubRect,
    pub ui_borders_3x3_4: SubRect,
    pub ui_borders_3x3_5: SubRect,
    pub ui_borders_3x3_6: SubRect,
    pub ui_borders_3x3_7: SubRect,
    pub ui_borders_3x3_8: SubRect,

    pub ui_background_box_3x3_0: SubRect,
    pub ui_background_box_3x3_1: SubRect,
    pub ui_background_box_3x3_2: SubRect,
    pub ui_background_box_3x3_3: SubRect,
    pub ui_background_box_3x3_4: SubRect,
    pub ui_background_box_3x3_5: SubRect,
    pub ui_background_box_3x3_6: SubRect,
    pub ui_background_box_3x3_7: SubRect,
    pub ui_background_box_3x3_8: SubRect,

    pub ui_selection_3x3_0: SubRect,
    pub ui_selection_3x3_1: SubRect,
    pub ui_selection_3x3_2: SubRect,
    pub ui_selection_3x3_3: SubRect,
    pub ui_selection_3x3_4: SubRect,
    pub ui_selection_3x3_5: SubRect,
    pub ui_selection_3x3_6: SubRect,
    pub ui_selection_3x3_7: SubRect,
    pub ui_selection_3x3_8: SubRect,

    pub button_3x3_idle_0: SubRect,
    pub button_3x3_idle_1: SubRect,
    pub button_3x3_idle_2: SubRect,
    pub button_3x3_idle_3: SubRect,
    pub button_3x3_idle_4: SubRect,
    pub button_3x3_idle_5: SubRect,
    pub button_3x3_idle_6: SubRect,
    pub button_3x3_idle_7: SubRect,
    pub button_3x3_idle_8: SubRect,

    pub button_3x3_clicked_0: SubRect,
    pub button_3x3_clicked_1: SubRect,
    pub button_3x3_clicked_2: SubRect,
    pub button_3x3_clicked_3: SubRect,
    pub button_3x3_clicked_4: SubRect,
    pub button_3x3_clicked_5: SubRect,
    pub button_3x3_clicked_6: SubRect,
    pub button_3x3_clicked_7: SubRect,
    pub button_3x3_clicked_8: SubRect,

    pub button_3x3_hover_0: SubRect,
    pub button_3x3_hover_1: SubRect,
    pub button_3x3_hover_2: SubRect,
    pub button_3x3_hover_3: SubRect,
    pub button_3x3_hover_4: SubRect,
    pub button_3x3_hover_5: SubRect,
    pub button_3x3_hover_6: SubRect,
    pub button_3x3_hover_7: SubRect,
    pub button_3x3_hover_8: SubRect,

    pub small_axe_icon: SubRect,
    pub huge_axe_icon: SubRect,
}
use {
    macroquad::prelude::*,
    simple_tiled_wfc::grid_generation::{WfcContext, WfcContextBuilder, WfcModule},
    std::{
        sync::Arc,
        sync::mpsc::{channel,}
    }
};

use crate::core_subsystems::atlas_serialization::*;
use crate::core_subsystems::types::*;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use simple_tiled_wfc::errors::WfcError;

const TILES: &[MainTile] = &[
    MainTile::Land_GrassSharp_0,
    MainTile::Land_GrassSharp_1,
    MainTile::Land_GrassSharp_2,
    MainTile::Land_GrassSharp_3,
    MainTile::Land_GrassSharp_4,
    MainTile::Land_GrassSharp_5,
    MainTile::Land_GrassSharp_6,
    MainTile::Land_GrassSharp_7,
    MainTile::Land_GrassSharp_8,
    MainTile::Land_GrassSharp_9,
    MainTile::Land_GrassSharp_10,
    MainTile::Land_GrassSharp_11,
    MainTile::Land_GrassSharp_12,
    MainTile::Land_GrassSharp_13,
    MainTile::Land_GrassSharp_14,
    MainTile::Land_GrassSharp_15,
    MainTile::Land_GrassSharp_16,
    MainTile::Land_GrassSharp_17,
    MainTile::Land_GrassSharp_18,
    MainTile::Land_GrassSharp_19,
    MainTile::Land_GrassSharp_20,
    MainTile::Land_GrassSharp_21,
    MainTile::Land_GrassSharp_22,
    MainTile::Land_GrassSharp_23,
    MainTile::Land_GrassSharp_24,
    MainTile::Land_GrassSharp_25,
    MainTile::Land_GrassSharp_26,
    MainTile::Land_GrassSharp_27,
    MainTile::Land_GrassSharp_28,
    MainTile::Land_GrassSharp_29,
    MainTile::Land_GrassSharp_30,
    MainTile::Land_GrassSharp_31,
    MainTile::Land_GrassSharp_32,
    MainTile::Land_GrassSharp_33,
    MainTile::Land_GrassSharp_34,
    MainTile::Land_GrassSharp_35,
    MainTile::Land_GrassSharp_36,
    MainTile::Land_GrassSharp_37,
    MainTile::Land_GrassSharp_38,
    MainTile::Land_GrassSharp_39,
    MainTile::Land_GrassSharp_40,
    MainTile::Land_GrassSharp_41,
    MainTile::Land_GrassSharp_42,
    MainTile::Land_GrassSharp_43,
    MainTile::Land_GrassSharp_44,
    MainTile::Land_GrassSharp_45,
    MainTile::Land_GrassSharp_46,
    MainTile::Land_GrassSharp_47,

    MainTile::GrassRound_GrassSharp_0,
    MainTile::GrassRound_GrassSharp_1,
    MainTile::GrassRound_GrassSharp_2,
    MainTile::GrassRound_GrassSharp_3,
    MainTile::GrassRound_GrassSharp_4,
    MainTile::GrassRound_GrassSharp_5,
    MainTile::GrassRound_GrassSharp_6,
    MainTile::GrassRound_GrassSharp_7,
    MainTile::GrassRound_GrassSharp_8,
    MainTile::GrassRound_GrassSharp_9,
    MainTile::GrassRound_GrassSharp_10,
    MainTile::GrassRound_GrassSharp_11,
    MainTile::GrassRound_GrassSharp_12,
    MainTile::GrassRound_GrassSharp_13,
    MainTile::GrassRound_GrassSharp_14,
    MainTile::GrassRound_GrassSharp_15,
    MainTile::GrassRound_GrassSharp_16,
    MainTile::GrassRound_GrassSharp_17,
    MainTile::GrassRound_GrassSharp_18,
    MainTile::GrassRound_GrassSharp_19,
    MainTile::GrassRound_GrassSharp_20,
    MainTile::GrassRound_GrassSharp_21,
    MainTile::GrassRound_GrassSharp_22,
    MainTile::GrassRound_GrassSharp_23,
    MainTile::GrassRound_GrassSharp_24,
    MainTile::GrassRound_GrassSharp_25,
    MainTile::GrassRound_GrassSharp_26,
    MainTile::GrassRound_GrassSharp_27,
    MainTile::GrassRound_GrassSharp_28,
    MainTile::GrassRound_GrassSharp_29,
    MainTile::GrassRound_GrassSharp_30,
    MainTile::GrassRound_GrassSharp_31,
    MainTile::GrassRound_GrassSharp_32,
    MainTile::GrassRound_GrassSharp_33,
    MainTile::GrassRound_GrassSharp_34,
    MainTile::GrassRound_GrassSharp_35,
    MainTile::GrassRound_GrassSharp_36,
    MainTile::GrassRound_GrassSharp_37,
    MainTile::GrassRound_GrassSharp_38,
    MainTile::GrassRound_GrassSharp_39,
    MainTile::GrassRound_GrassSharp_40,
    MainTile::GrassRound_GrassSharp_41,
    MainTile::GrassRound_GrassSharp_42,
    MainTile::GrassRound_GrassSharp_43,
    MainTile::GrassRound_GrassSharp_44,
    MainTile::GrassRound_GrassSharp_45,
    MainTile::GrassRound_GrassSharp_46,
    MainTile::GrassRound_GrassSharp_47,

    MainTile::Land_GrassRound_0,
    MainTile::Land_GrassRound_1,
    MainTile::Land_GrassRound_2,
    MainTile::Land_GrassRound_3,
    MainTile::Land_GrassRound_4,
    MainTile::Land_GrassRound_5,
    MainTile::Land_GrassRound_6,
    MainTile::Land_GrassRound_7,
    MainTile::Land_GrassRound_8,
    MainTile::Land_GrassRound_9,
    MainTile::Land_GrassRound_10,
    MainTile::Land_GrassRound_11,
    MainTile::Land_GrassRound_12,
    MainTile::Land_GrassRound_13,
    MainTile::Land_GrassRound_14,
    MainTile::Land_GrassRound_15,
    MainTile::Land_GrassRound_16,
    MainTile::Land_GrassRound_17,
    MainTile::Land_GrassRound_18,
    MainTile::Land_GrassRound_19,
    MainTile::Land_GrassRound_20,
    MainTile::Land_GrassRound_21,
    MainTile::Land_GrassRound_22,
    MainTile::Land_GrassRound_23,
    MainTile::Land_GrassRound_24,
    MainTile::Land_GrassRound_25,
    MainTile::Land_GrassRound_26,
    MainTile::Land_GrassRound_27,
    MainTile::Land_GrassRound_28,
    MainTile::Land_GrassRound_29,
    MainTile::Land_GrassRound_30,
    MainTile::Land_GrassRound_31,
    MainTile::Land_GrassRound_32,
    MainTile::Land_GrassRound_33,
    MainTile::Land_GrassRound_34,
    MainTile::Land_GrassRound_35,
    MainTile::Land_GrassRound_36,
    MainTile::Land_GrassRound_37,
    MainTile::Land_GrassRound_38,
    MainTile::Land_GrassRound_39,
    MainTile::Land_GrassRound_40,
    MainTile::Land_GrassRound_41,
    MainTile::Land_GrassRound_42,
    MainTile::Land_GrassRound_43,
    MainTile::Land_GrassRound_44,
    MainTile::Land_GrassRound_45,
    MainTile::Land_GrassRound_46,
    MainTile::Land_GrassRound_47,

    MainTile::Land_Water_0,
    MainTile::Land_Water_1,
    MainTile::Land_Water_2,
    MainTile::Land_Water_3,
    MainTile::Land_Water_4,
    MainTile::Land_Water_5,
    MainTile::Land_Water_6,
    MainTile::Land_Water_7,
    MainTile::Land_Water_8,
    MainTile::Land_Water_9,
    MainTile::Land_Water_10,
    MainTile::Land_Water_11,
    MainTile::Land_Water_12,
    MainTile::Land_Water_13,
    MainTile::Land_Water_14,
    MainTile::Land_Water_15,
    MainTile::Land_Water_16,
    MainTile::Land_Water_17,
    MainTile::Land_Water_18,
    MainTile::Land_Water_19,
    MainTile::Land_Water_20,
    MainTile::Land_Water_21,
    MainTile::Land_Water_22,
    MainTile::Land_Water_23,
    MainTile::Land_Water_24,
    MainTile::Land_Water_25,
    MainTile::Land_Water_26,
    MainTile::Land_Water_27,
    MainTile::Land_Water_28,
    MainTile::Land_Water_29,
    MainTile::Land_Water_30,
    MainTile::Land_Water_31,
    MainTile::Land_Water_32,
    MainTile::Land_Water_33,
    MainTile::Land_Water_34,
    MainTile::Land_Water_35,
    MainTile::Land_Water_36,
    MainTile::Land_Water_37,
    MainTile::Land_Water_38,
    MainTile::Land_Water_39,
    MainTile::Land_Water_40,
    MainTile::Land_Water_41,
    MainTile::Land_Water_42,
    MainTile::Land_Water_43,
    MainTile::Land_Water_44,
    MainTile::Land_Water_45,
    MainTile::Land_Water_46,
    MainTile::Land_Water_47,

    MainTile::VerticalBridge_0,
    MainTile::VerticalBridge_1,
    MainTile::VerticalBridge_2,
    MainTile::VerticalBridge_3,
    MainTile::VerticalBridge_4,
    MainTile::VerticalBridge_5,
    MainTile::VerticalBridge_6,
    MainTile::VerticalBridge_7,
    MainTile::VerticalBridge_8,

    MainTile::HorizontalBridge_0,
    MainTile::HorizontalBridge_1,
    MainTile::HorizontalBridge_2,
    MainTile::HorizontalBridge_3,
    MainTile::HorizontalBridge_4,
    MainTile::HorizontalBridge_5,
    MainTile::HorizontalBridge_6,
    MainTile::HorizontalBridge_7,
    MainTile::HorizontalBridge_8,
];

pub struct Tilemap {
    pub w: usize,
    pub h: usize,
    pub map_data: Vec<usize>,
    pub tiles: Vec<MainTile>,
    pub tile_sides: Vec<TileSides>,
    is_generating: bool,
    tx: Sender<Result<Vec<usize>, WfcError>>,
    rc: Receiver<Result<Vec<usize>, WfcError>>,
    modules: Vec<WfcModule<CustomBitSet>>
}

impl Tilemap {
    pub fn new(
        atlas_definition: Arc<AtlasScheme>,
        w: usize,
        h: usize
    ) -> Self {
        let (tx, rc) = channel();
        let mut tile_sides = Vec::new();
        let mut neighbour_strategies = Vec::new();
        let mut tiles = Vec::new();
        {
            for tile_cfg in atlas_definition.terrain_tile_configs.iter() {
                for pattern in &[
                    &atlas_definition.reduced_wang_patterns[..],
                    &atlas_definition.extended_set_1_patterns_north_west[..],
                    &atlas_definition.extended_set_1_patterns_north_east[..],
                    &atlas_definition.extended_set_1_patterns_south_west[..],
                    &atlas_definition.extended_set_1_patterns_south_east[..],
                    &atlas_definition.extended_set_2_patterns_north_west[..],
                    &atlas_definition.extended_set_2_patterns_north_east[..],
                    &atlas_definition.extended_set_2_patterns_south_west[..],
                    &atlas_definition.extended_set_2_patterns_south_east[..],
                ] {
                    tile_sides.extend(
                        pattern.iter().map(
                            |pattern| {
                                TileSides {
                                    north_west: match pattern.north_west {
                                        TileKind::Inner => { tile_cfg.inner_type }
                                        TileKind::Outer => { tile_cfg.outer_type }
                                    },
                                    north_east: match pattern.north_east {
                                        TileKind::Inner => { tile_cfg.inner_type }
                                        TileKind::Outer => { tile_cfg.outer_type }
                                    },
                                    south_west: match pattern.south_west {
                                        TileKind::Inner => { tile_cfg.inner_type }
                                        TileKind::Outer => { tile_cfg.outer_type }
                                    },
                                    south_east: match pattern.south_east {
                                        TileKind::Inner => { tile_cfg.inner_type }
                                        TileKind::Outer => { tile_cfg.outer_type }
                                    },
                                }
                            }
                        )
                    );
                }
                for j in 0..4 {
                    for i in 0..4 {
                        tiles.push(TILES[i + j * 4 + tile_cfg.x_offset * 48]);
                    }
                }
                for offs in &[16, 32] {
                    for jj in 0..2 {
                        for ii in 0..2 {
                            for j in 0..2 {
                                for i in 0..2 {
                                    tiles.push(TILES[offs + i + ii * 2 + (j + jj * 2) * 4 + tile_cfg.x_offset * 48]);
                                }
                            }
                        }
                    }
                }
                neighbour_strategies.extend_from_slice(&atlas_definition.reduced_wang_neighbour_strategy[..]);
                for _ in 0..8 {
                    neighbour_strategies.extend_from_slice(&atlas_definition.neighbour_strategy_2_x_2[..]);
                }
            }
            tile_sides.extend_from_slice(&atlas_definition.vertical_bridge_sides[..]);
            tile_sides.extend_from_slice(&atlas_definition.horizontal_bridge_sides[..]);
            tiles.extend_from_slice(&[
                MainTile::VerticalBridge_0,
                MainTile::VerticalBridge_1,
                MainTile::VerticalBridge_2,
                MainTile::VerticalBridge_3,
                MainTile::VerticalBridge_4,
                MainTile::VerticalBridge_5,
                MainTile::VerticalBridge_6,
                MainTile::VerticalBridge_7,
                MainTile::VerticalBridge_8,

                MainTile::HorizontalBridge_0,
                MainTile::HorizontalBridge_1,
                MainTile::HorizontalBridge_2,
                MainTile::HorizontalBridge_3,
                MainTile::HorizontalBridge_4,
                MainTile::HorizontalBridge_5,
                MainTile::HorizontalBridge_6,
                MainTile::HorizontalBridge_7,
                MainTile::HorizontalBridge_8,
            ]);
            neighbour_strategies.extend_from_slice(&atlas_definition.neighbour_strategy_3_x_3[..]);
            neighbour_strategies.extend_from_slice(&atlas_definition.neighbour_strategy_3_x_3[..]);

            assert_eq!(tile_sides.len(), neighbour_strategies.len());
        }

        let mut modules = Vec::new();
        for i in 0..tile_sides.len() {
            let current_sides = tile_sides[i];
            let mut module = WfcModule::new();
            match neighbour_strategies[i].north {
                NeighbourKind::WangCorners => {
                    for j in 0..tile_sides.len() {
                        if neighbour_strategies[j].south != NeighbourKind::WangCorners {
                            continue;
                        }
                        if tile_sides[j].south_west == current_sides.north_west &&
                            tile_sides[j].south_east == current_sides.north_east {
                            module.add_north_neighbour(j);
                        }
                    }
                }
                NeighbourKind::RelOffset(offset) => {
                    let new_offset = (i as i32 + offset) as usize;
                    module.add_north_neighbour(new_offset);
                }
            }
            match neighbour_strategies[i].south {
                NeighbourKind::WangCorners => {
                    for j in 0..tile_sides.len() {
                        if neighbour_strategies[j].north != NeighbourKind::WangCorners {
                            continue;
                        }
                        if tile_sides[j].north_west == current_sides.south_west &&
                            tile_sides[j].north_east == current_sides.south_east {
                            module.add_south_neighbour(j);
                        }
                    }
                }
                NeighbourKind::RelOffset(offset) => {
                    let new_offset = (i as i32 + offset) as usize;
                    module.add_south_neighbour(new_offset);
                }
            }
            match neighbour_strategies[i].east {
                NeighbourKind::WangCorners => {
                    for j in 0..tile_sides.len() {
                        if neighbour_strategies[j].west != NeighbourKind::WangCorners {
                            continue;
                        }
                        if tile_sides[j].north_west == current_sides.north_east &&
                            tile_sides[j].south_west == current_sides.south_east {
                            module.add_east_neighbour(j);
                        }
                    }
                }
                NeighbourKind::RelOffset(offset) => {
                    let new_offset = (i as i32 + offset) as usize;
                    module.add_east_neighbour(new_offset);
                }
            }
            match neighbour_strategies[i].west {
                NeighbourKind::WangCorners => {
                    for j in 0..tile_sides.len() {
                        if neighbour_strategies[j].east != NeighbourKind::WangCorners {
                            continue;
                        }
                        if tile_sides[j].north_east == current_sides.north_west &&
                            tile_sides[j].south_east == current_sides.south_west {
                            module.add_west_neighbour(j);
                        }
                    }
                }
                NeighbourKind::RelOffset(offset) => {
                    let new_offset = (i as i32 + offset) as usize;
                    module.add_west_neighbour(new_offset);
                }
            }
            modules.push(module);
        }

        Self {
            is_generating: false,
            w,
            h,
            tiles,
            tile_sides,
            modules,
            tx,
            rc,
            map_data: vec![0; w * h],
        }
    }

    pub fn generate_new_map(&mut self) {
        self.is_generating = true;

        let modules = self.modules.clone();
        let w = self.w;
        let h = self.h;

        let tx2 = self.tx.clone();

        thread::spawn(
            move||{
                let mut wfc_context: WfcContext<CustomBitSet> = WfcContextBuilder
                ::new(&modules, w, h)
                    .build();
                wfc_context.collapse(100, tx2);
            }
        );
    }

    pub fn is_busy(&self) -> bool {self.is_generating }

    pub fn poll(&mut self) -> bool {
        match self.rc.try_recv() {
            Ok(Ok(r)) => {
                self.map_data.clear();
                self.map_data.extend_from_slice(&r[..]);
                self.is_generating = false;
                true
            }
            Ok(Err(_)) => {
                self.map_data.fill(0);
                self.is_generating = false;
                true
            }
            _ => {
                false
            }
        }
    }
}
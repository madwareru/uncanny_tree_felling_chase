use {
    macroquad::prelude::*,
    simple_tiled_wfc::grid_generation::{WfcContext, WfcContextBuilder, WfcModule},
    std::{
        sync::Arc,
        sync::mpsc::channel
    }
};

use crate::core_subsystems::atlas_serialization::*;
use crate::core_subsystems::types::*;

pub struct Tilemap {
    pub w: usize,
    pub h: usize,
    pub map_data: Vec<usize>,
    pub tiles: Vec<SubRect>,
    pub tile_sides: Vec<TileSides>,
    modules: Vec<WfcModule<CustomBitSet>>
}

impl Tilemap {
    pub fn new(
        atlas_definition: Arc<AtlasDefinition>,
        w: usize,
        h: usize
    ) -> Self {
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
                        tiles.push(SubRect {
                            x: i * atlas_definition.tile_width as i32 + tile_cfg.x_offset,
                            y: j * atlas_definition.tile_height as i32 + tile_cfg.y_offset,
                            width: atlas_definition.tile_width as i32,
                            height: atlas_definition.tile_height as i32,
                        })
                    }
                }
                for offsets in &[
                    (tile_cfg.x_offset + 1024, tile_cfg.y_offset),
                    (tile_cfg.x_offset, tile_cfg.y_offset + 1024)
                ] {
                    for jj in 0..2 {
                        for ii in 0..2 {
                            for j in 0..2 {
                                for i in 0..2 {
                                    tiles.push(SubRect {
                                        x: (i + ii * 2) * atlas_definition.tile_width as i32 + offsets.0,
                                        y: (j + jj * 2) * atlas_definition.tile_height as i32 + offsets.1,
                                        width: atlas_definition.tile_width as i32,
                                        height: atlas_definition.tile_height as i32,
                                    })
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
            for j in 0..3 {
                for i in 0..3 {
                    tiles.push(SubRect {
                        x: (i + 8) * atlas_definition.tile_width as i32,
                        y: (j + 8) * atlas_definition.tile_height as i32,
                        width: atlas_definition.tile_width as i32,
                        height: atlas_definition.tile_height as i32,
                    })
                }
            }
            for j in 0..3 {
                for i in 0..3 {
                    tiles.push(SubRect {
                        x: (i + 11) * atlas_definition.tile_width as i32,
                        y: (j + 8) * atlas_definition.tile_height as i32,
                        width: atlas_definition.tile_width as i32,
                        height: atlas_definition.tile_height as i32,
                    })
                }
            }
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
            w,
            h,
            tiles,
            tile_sides,
            modules,
            map_data: vec![0; w * h],
        }
    }

    pub fn generate_new_map(&mut self) {
        let mut wfc_context: WfcContext<CustomBitSet> = WfcContextBuilder
            ::new(&self.modules, self.w, self.h)
            .build();

        let (tx, rc) = channel();

        wfc_context.collapse(100, tx.clone());

        let results = rc.recv()
            .unwrap()
            .unwrap_or_else(|_| vec![0; self.w * self.h]);

        self.map_data.clear();
        self.map_data.extend_from_slice(&results[..]);
    }
}
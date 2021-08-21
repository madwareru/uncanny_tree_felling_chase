use {
    macroquad::prelude::*,
    std::collections::HashMap
};

use crate::core_subsystems::atlas_serialization::TreeType;
use crate::core_subsystems::tilemap::Tilemap;
use std::sync::Arc;
use crate::core_subsystems::units_serialization::UnitsConfig;

#[derive(Copy, Clone)]
pub struct Tree{
    pub tree_type: TreeType,
    pub health: i32
}
impl Tree {
    pub const NONE: Self = Tree {
        tree_type: TreeType::None,
        health: 0
    };
}

pub struct Forest {
    pub corner_tree_data: Vec<Tree>,
    pub cell_tree_data: Vec<Tree>,
    units_config: Arc<UnitsConfig>,
    tree_probabilities: HashMap<TreeType, i32>,
    total_planted: usize
}

impl Forest {
    pub fn create(tilemap: &Tilemap, units_config: Arc<UnitsConfig>) -> Self {
        Self {
            units_config,
            total_planted: 0,
            corner_tree_data: vec![Tree::NONE; tilemap.w * tilemap.h],
            cell_tree_data: vec![Tree::NONE; tilemap.w * tilemap.h],
            tree_probabilities: {
                let mut map = HashMap::new();
                map.insert(TreeType::Bush, 4);
                map.insert(TreeType::Oak, 12);
                map.insert(TreeType::Pine, 16);
                map
            }
        }
    }

    pub fn plant_trees(&mut self, tilemap: &Tilemap) {
        const FOREST_TILE_OFFSET_START: usize = 48;
        const FOREST_TILE_OFFSET_END: usize = 95;

        self.total_planted = 0;
        self.cell_tree_data.fill(Tree::NONE);
        self.corner_tree_data.fill(Tree::NONE);

        for j in 1..tilemap.h - 1 {
            for i in 1..tilemap.w - 1 {
                let idx = j * tilemap.w + i;
                if tilemap.map_data[idx] >= FOREST_TILE_OFFSET_START &&
                    tilemap.map_data[idx] <= FOREST_TILE_OFFSET_END
                {
                    self.try_plant_cell_tree(idx);
                    self.try_plant_corner_tree(idx);
                }
            }
        }
    }

    fn try_plant(&mut self) -> TreeType {
        self.try_plant_tree_type(TreeType::Bush)
            .unwrap_or_else(|| {
                self.try_plant_tree_type(TreeType::Oak)
                    .unwrap_or_else(|| {
                        self.try_plant_tree_type(TreeType::Pine)
                            .unwrap_or(TreeType::None) }) })
    }

    fn try_plant_tree_type(&mut self, tree_type: TreeType) -> Option<TreeType> {
        self.tree_probabilities.get(&tree_type)
            .and_then(
                |&probability| if probability >= rand::gen_range(0, 100) {
                    Some(tree_type)
                } else {
                    None
                }
            )
    }

    fn make_tree(&self, tree_type: TreeType) -> Tree {
        self.units_config.tree_health
            .get(&tree_type)
            .map(| it | Tree {
                tree_type,
                health: *it
            })
            .unwrap_or( Tree {
                tree_type,
                health: 0
            })
    }

    fn try_plant_cell_tree(&mut self, idx: usize) {
        let plant = self.try_plant();
        self.cell_tree_data[idx] = self.make_tree(plant);
        self.total_planted += if self.cell_tree_data[idx].tree_type != TreeType::None { 1 } else { 0 };
    }

    fn try_plant_corner_tree(&mut self, idx: usize) {
        let plant = self.try_plant();
        self.corner_tree_data[idx] = self.make_tree(plant);
        self.total_planted += if self.corner_tree_data[idx].tree_type != TreeType::None { 1 } else { 0 };
    }

    pub fn total_planted(&self) -> usize { self.total_planted }
}
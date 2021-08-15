use {
    macroquad::prelude::*,
    std::collections::HashMap
};

use crate::core_subsystems::atlas_serialization::TreeType;
use crate::core_subsystems::tilemap::Tilemap;

pub struct Forest {
    pub corner_tree_data: Vec<TreeType>,
    pub cell_tree_data: Vec<TreeType>,
    tree_probabilities: HashMap<TreeType, i32>,
}

impl Forest {
    pub fn create(tilemap: &Tilemap) -> Self {
        Self {
            corner_tree_data: vec![TreeType::None; tilemap.w * tilemap.h],
            cell_tree_data: vec![TreeType::None; tilemap.w * tilemap.h],
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

        self.cell_tree_data.fill(TreeType::None);
        self.corner_tree_data.fill(TreeType::None);

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

    fn try_plant_cell_tree(&mut self, idx: usize) {
        self.cell_tree_data[idx] = self.try_plant();
    }

    fn try_plant_corner_tree(&mut self, idx: usize) {
        self.corner_tree_data[idx] = self.try_plant();
    }
}
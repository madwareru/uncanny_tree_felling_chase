#[derive(Copy, Clone, PartialEq)]
pub enum MapFieldOccupation {
    Unoccupied,
    Blocked,
    MinionLanded{
        is_big: bool,
        entity: hecs::Entity
    }
}

pub struct MapFieldOccupationData {
    width: usize,
    height: usize,
    data: Vec<MapFieldOccupation>
}

impl MapFieldOccupationData {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![MapFieldOccupation::Unoccupied; width*height]
        }
    }
    pub fn clear(&mut self) {
        self.data.fill(MapFieldOccupation::Unoccupied)
    }
    pub fn clear_minions(&mut self) {
        for entry in self.data.iter_mut() {
            if let MapFieldOccupation::MinionLanded { entity, .. } = entry {
                *entry = MapFieldOccupation::Unoccupied;
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> MapFieldOccupation {
        assert!(x < self.width && y < self.height);
        self.data[y * self.width + x]
    }
    pub fn take_mut(&mut self, x: usize, y: usize) -> &mut MapFieldOccupation {
        assert!(x < self.width && y < self.height);
        &mut self.data[y * self.width + x]
    }
}
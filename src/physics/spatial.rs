use crate::ecs::Entity;
use glam::Vec2;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SpatialHash {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn insert(&mut self, entity: Entity, position: Vec2, bounds: (Vec2, Vec2)) {
        let min_cell = self.position_to_cell(position + bounds.0);
        let max_cell = self.position_to_cell(position + bounds.1);

        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                self.cells
                    .entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(entity);
            }
        }
    }

    pub fn query(&self, position: Vec2, bounds: (Vec2, Vec2)) -> Vec<Entity> {
        let min_cell = self.position_to_cell(position + bounds.0);
        let max_cell = self.position_to_cell(position + bounds.1);

        let mut result = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                if let Some(entities) = self.cells.get(&(x, y)) {
                    for &entity in entities {
                        if seen.insert(entity) {
                            result.push(entity);
                        }
                    }
                }
            }
        }

        result
    }

    fn position_to_cell(&self, position: Vec2) -> (i32, i32) {
        (
            (position.x / self.cell_size).floor() as i32,
            (position.y / self.cell_size).floor() as i32,
        )
    }
}

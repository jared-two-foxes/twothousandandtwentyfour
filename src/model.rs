use rand::prelude::*;

use crate::grid::Grid;
use crate::vec2::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Running,
    Won,
    WonContinue,
    Lost,
}

pub struct Model {
    // Tile values store exponents: 0 = empty, 1 = 2, 2 = 4, ..., 11 = 2048.
    pub grid: Grid<u16>,
    rng: rand::rngs::ThreadRng,
    pub state: State,
    pub score: u32,
}

impl Model {
    pub fn new() -> Self {
        let mut model = Model {
            grid: Grid::new(4, 4),
            rng: rand::rng(),
            state: State::Running,
            score: 0,
        };
        for _ in 0..2 {
            model.generate_new_value();
        }
        model
    }

    pub fn empty_nodes(&self) -> Vec<Vec2> {
        // The game board is intentionally fixed at 4x4.
        let mut empty_nodes = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if *self.grid.value(i, j) == 0 {
                    empty_nodes.push(Vec2::new(i as i32, j as i32));
                }
            }
        }
        empty_nodes
    }

    fn adjacent_by_column(&self) -> bool {
        for c in self.grid.columns() {
            for i in 1..3 {
                if c[i] == c[i - 1] {
                    return true;
                }
            }
        }
        false
    }

    fn adjacent_by_row(&self) -> bool {
        for r in self.grid.rows() {
            for i in 1..3 {
                if r[i] == r[i - 1] {
                    return true;
                }
            }
        }
        false
    }

    // Returns true if at least one legal move remains.
    pub fn check_for_valid_moves(&self) -> bool {
        !self.empty_nodes().is_empty() || self.adjacent_by_column() || self.adjacent_by_row()
    }

    fn pick_new_value(&mut self) -> u16 {
        if self.rng.random_ratio(9, 10) {
            1
        } else {
            2
        }
    }

    pub fn generate_new_value(&mut self) -> bool {
        // find an empty square, and add a value to it.
        let mut empty_nodes = self.empty_nodes();
        empty_nodes.shuffle(&mut self.rng);
        match empty_nodes.choose(&mut self.rng) {
            Some(node) => {
                let value = self.pick_new_value();
                let v = self
                    .grid
                    .get_mut(node.x as usize, node.y as usize)
                    .expect("Unable to find value for node: {node}");
                *v = value;
                true
            }
            None => false,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn non_zero_tiles(model: &Model) -> Vec<u16> {
        let mut values = Vec::new();
        for i in 0..model.grid.height() {
            for j in 0..model.grid.width() {
                let value = *model.grid.value(i, j);
                if value != 0 {
                    values.push(value);
                }
            }
        }
        values
    }

    #[test]
    fn new_spawns_two_tiles_with_exponent_values() {
        let model = Model::new();
        let values = non_zero_tiles(&model);

        assert_eq!(values.len(), 2);
        assert!(values.iter().all(|v| *v == 1 || *v == 2));
    }

    #[test]
    fn generate_new_value_uses_exponents_and_stops_when_full() {
        let mut model = Model::new();

        for i in 0..model.grid.height() {
            for j in 0..model.grid.width() {
                *model.grid.value_mut(i, j) = 0;
            }
        }

        for _ in 0..16 {
            assert!(model.generate_new_value());
        }

        let values = non_zero_tiles(&model);
        assert_eq!(values.len(), 16);
        assert!(values.iter().all(|v| *v == 1 || *v == 2));

        assert!(!model.generate_new_value());
    }
}



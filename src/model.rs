use rand::prelude::*;

use crate::grid::Grid;
use crate::vec2::Vec2;

#[derive(Default)]
pub struct Model {
    pub grid: Grid<u16>,
    rng: rand::rngs::ThreadRng,
    pub score: u32,
}

impl Model {
    pub fn new() -> Self {
        let mut model = Model {
            grid: Grid::new(4, 4),
            rng: rand::rng(),
            score: 0,
        };
        for _ in 0..2 {
            model.generate_new_value();
        }
        model
    }

    pub fn empty_nodes(&self) -> Vec<Vec2> {
        // find an empty square, and add a value to it.
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

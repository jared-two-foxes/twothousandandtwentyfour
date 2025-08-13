use crate::grid::Grid;

#[derive(Default)]
pub struct Model {
    pub grid: Grid<u16>,
    pub score: u32,
}

impl Model {
    pub fn new() -> Self {
        Model {
            grid: Grid::new(4, 4),
            score: 0,
        }
    }
}

use crate::grid::Grid;

#[derive(Default)]
pub struct Model {
    pub grid: Grid<u16>,
    pub score: u32,
}

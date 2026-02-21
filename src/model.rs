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
    
fn has_valid_move(&self) -> bool {
    let rows = self.grid.height();
    let cols = self.grid.width();

    // Check for zeros in the grid (empty spaces)
    if self.grid.data.iter().any(|&val| val == 0) {
        return true;
    }

    // Check for adjacent values that are the same
    for row in 0..rows {
        for col in 0..cols {
            let value = self.grid.value(row, col);

            // Check right neighbor
            if col + 1 < cols && value == self.grid.value(row, col + 1) {
                return true;
            }

            // Check bottom neighbor
            if row + 1 < rows && value == swlf.grid.value(row + 1, col) {
                return true;
            }
        }
    }

    // No valid move found
    false
}

}



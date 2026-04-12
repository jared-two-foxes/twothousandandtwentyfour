use std::ops::Index;
use std::ops::IndexMut;

#[derive(Default)]
pub struct Grid<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub struct Row<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
}

impl<'a, T> Row<'a, T> {
    #[inline]
    fn get(&self, index: usize) -> &T {
        self.grid.value(self.row, index)
    }
}

impl<'a, T> Index<usize> for Row<'a, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index)
    }
}

pub struct RowMut<'a, T> {
    grid: &'a mut Grid<T>,
    row: usize,
}

impl<'a, T> RowMut<'a, T> {
    #[inline]
    fn get(&self, index: usize) -> &T {
        self.grid.value(self.row, index)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> &mut T {
        self.grid.value_mut(self.row, index)
    }
}

impl<'a, T> Index<usize> for RowMut<'a, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index)
    }
}

impl<'a, T> IndexMut<usize> for RowMut<'a, T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index)
    }
}

//@todo: could impl an iterator object that is created from GridRow, a GridRowIter
/*struct RowIterator<'a, T> {
    row: &'a Row<'a, T>,
    index: usize,
}

impl<'a, T> Iterator for GridRow<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row > self.grid.rows().len() {
            return None;
        }
        let n = self.grid.get(self.row, self.index);
        self.index += 1;
        n
    }
}*/

pub struct Column<'a, T> {
    grid: &'a Grid<T>,
    column: usize,
}

impl<'a, T> Column<'a, T> {
    #[inline]
    #[must_use]
    fn get(&self, index: usize) -> &T {
        self.grid.value(index, self.column)
    }
}

impl<'a, T> Index<usize> for Column<'a, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index)
    }
}

pub struct ColumnMut<'a, T> {
    grid: &'a mut Grid<T>,
    column: usize,
}

impl<'a, T> ColumnMut<'a, T> {
    #[inline]
    #[must_use]
    fn get(&self, index: usize) -> &T {
        self.grid.value(index, self.column)
    }

    fn get_mut(&mut self, index: usize) -> &mut T {
        self.grid.value_mut(index, self.column)
    }
}

impl<'a, T> Index<usize> for ColumnMut<'a, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index)
    }
}

impl<'a, T> IndexMut<usize> for ColumnMut<'a, T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index)
    }
}

impl<T> Grid<T> {
    pub fn new(r: usize, c: usize) -> Self
    where
        T: Default + Clone,
    {
        let mut data = Vec::new();
        data.resize(r * c, Default::default());
        Grid {
            data,
            rows: r,
            cols: c,
        }
    }

    pub fn width(&self) -> usize {
        self.rows
    }

    pub fn height(&self) -> usize {
        self.cols
    }

    pub fn row<'a>(&'a self, row: usize) -> Row<'a, T> {
        assert!(row < self.rows, "Index out of bounds");
        Row {
            grid: self,
            row,
        }
    }

    pub fn row_mut<'a>(&'a mut self, row: usize) -> RowMut<'a, T> {
        assert!(row < self.rows, "Index out of bounds");
        RowMut {
            grid: self,
            row,
        }
    }

    pub fn rows<'a>(&'a self) -> Vec<Row<'a, T>> {
        (0..self.rows).map(|i| Row { grid: self, row: i}).collect()
    }

    pub fn column<'a>(&'a self, col: usize) -> Column<'a, T> {
        assert!(col < self.cols, "Index out of bounds");
        Column {
            grid: self,
            column: col,
        }
    }

    pub fn column_mut<'a>(&'a mut self, col: usize) -> ColumnMut<'a, T> {
        assert!(col < self.cols, "Index out of bounds");
        ColumnMut {
            grid: self,
            column: col,
        }
    }

    pub fn columns<'a>(&'a self) -> Vec<Column<'a, T>> {
        (0..self.cols).map(|i| Column { grid: self, column: i}).collect()
    }

    #[inline]
    #[must_use]
    const fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        let index = self.get_index(row, col);
        self.data.get_unchecked(index)
    }

    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        let index = self.get_index(row, col);
        self.data.get_unchecked_mut(index)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            unsafe { Some(self.get_unchecked(row, col)) }
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            unsafe { Some(self.get_unchecked_mut(row, col)) }
        }
    }

    pub fn swap(&mut self, (row_a, col_a): (usize, usize), (row_b, col_b): (usize, usize)) {
        assert!(row_a < self.rows && col_a < self.cols, "First index out of bounds");
        assert!(row_b < self.rows && col_b < self.cols, "Second index out of bounds");
        let l = self.get_index(row_a, col_a);
        let r = self.get_index(row_b, col_b);
        self.data.swap(l, r);
    }

    pub fn value(&self, row: usize, column: usize) -> &T {
        assert!(row < self.rows, "Index out of bounds");
        assert!(column < self.cols, "Index out of bounds");
        let idx = self.get_index(row, column);
        &self.data[idx]
    }

    pub fn value_mut(&mut self, row: usize, column: usize) -> &mut T {
        assert!(row < self.rows, "Index out of bounds");
        assert!(column < self.cols, "Index out of bounds");
        let idx = self.get_index(row, column);
        &mut self.data[idx]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    #[inline]
    fn index(&self, (row, col): (usize, usize)) -> &T {
        assert!(row < self.rows && col < self.cols, "");
        let index = self.get_index(row, col);
        &self.data[index]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        assert!(row < self.rows && col < self.cols, "");
        let index = self.get_index(row, col);
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_columns_iteration() {
        let grid: Grid<u32> = {
            let mut g = Grid::new(2, 3);
            // Initialize grid with pattern: row-major order
            // [0, 1, 2]
            // [3, 4, 5]
            for i in 0..2 {
                for j in 0..3 {
                    *g.value_mut(i, j) = (i * 3 + j) as u32;
                }
            }
            g
        };

        let columns = grid.columns();
        assert_eq!(columns.len(), 3, "Should have 3 columns");

        // Column 0: [0, 3]
        assert_eq!(columns[0][0], 0);
        assert_eq!(columns[0][1], 3);

        // Column 1: [1, 4]
        assert_eq!(columns[1][0], 1);
        assert_eq!(columns[1][1], 4);

        // Column 2: [2, 5]
        assert_eq!(columns[2][0], 2);
        assert_eq!(columns[2][1], 5);
    }

    #[test]
    fn test_columns_single_column() {
        let grid: Grid<i32> = {
            let mut g = Grid::new(4, 1);
            for i in 0..4 {
                *g.value_mut(i, 0) = (i * 10) as i32;
            }
            g
        };

        let columns = grid.columns();
        assert_eq!(columns.len(), 1);
        assert_eq!(columns[0][0], 0);
        assert_eq!(columns[0][1], 10);
        assert_eq!(columns[0][2], 20);
        assert_eq!(columns[0][3], 30);
    }

    #[test]
    fn test_columns_single_row() {
        let grid: Grid<u16> = {
            let mut g = Grid::new(1, 4);
            for j in 0..4 {
                *g.value_mut(0, j) = (j as u16) * 5;
            }
            g
        };

        let columns = grid.columns();
        assert_eq!(columns.len(), 4);
        assert_eq!(columns[0][0], 0);
        assert_eq!(columns[1][0], 5);
        assert_eq!(columns[2][0], 10);
        assert_eq!(columns[3][0], 15);
    }

    #[test]
    fn test_rows_and_columns_consistency() {
        let grid: Grid<u32> = {
            let mut g = Grid::new(3, 4);
            // Fill with unique values
            for i in 0..3 {
                for j in 0..4 {
                    *g.value_mut(i, j) = (i * 10 + j) as u32;
                }
            }
            g
        };

        let rows = grid.rows();
        let columns = grid.columns();

        // Verify that grid[i][j] == rows[i][j] == columns[j][i]
        for i in 0..3 {
            for j in 0..4 {
                let val = *grid.value(i, j);
                assert_eq!(rows[i][j], val, "Row access mismatch at ({}, {})", i, j);
                assert_eq!(columns[j][i], val, "Column access mismatch at ({}, {})", i, j);
            }
        }
    }

    #[test]
    fn test_swap_basic() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        *grid.value_mut(0, 0) = 10;
        *grid.value_mut(0, 1) = 20;
        *grid.value_mut(1, 0) = 30;
        *grid.value_mut(1, 1) = 40;

        grid.swap((0, 0), (1, 1));

        assert_eq!(*grid.value(0, 0), 40);
        assert_eq!(*grid.value(1, 1), 10);
        // Other values unchanged
        assert_eq!(*grid.value(0, 1), 20);
        assert_eq!(*grid.value(1, 0), 30);
    }

    #[test]
    fn test_swap_same_position() {
        let mut grid: Grid<u8> = Grid::new(2, 2);
        *grid.value_mut(0, 0) = 42;
        *grid.value_mut(0, 1) = 99;

        grid.swap((0, 0), (0, 0));

        // Value should remain the same
        assert_eq!(*grid.value(0, 0), 42);
    }

    #[test]
    #[should_panic(expected = "First index out of bounds")]
    fn test_swap_validates_first_index_row() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        grid.swap((5, 0), (0, 0)); // row out of bounds
    }

    #[test]
    #[should_panic(expected = "First index out of bounds")]
    fn test_swap_validates_first_index_col() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        grid.swap((0, 5), (0, 0)); // col out of bounds
    }

    #[test]
    #[should_panic(expected = "Second index out of bounds")]
    fn test_swap_validates_second_index_row() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        grid.swap((0, 0), (5, 0)); // row out of bounds
    }

    #[test]
    #[should_panic(expected = "Second index out of bounds")]
    fn test_swap_validates_second_index_col() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        grid.swap((0, 0), (0, 5)); // col out of bounds
    }

    // ========== Grid creation and basic properties ==========

    #[test]
    fn test_grid_new() {
        let grid: Grid<i32> = Grid::new(3, 4);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 4);
    }

    #[test]
    fn test_grid_new_single_row() {
        let grid: Grid<u32> = Grid::new(1, 5);
        assert_eq!(grid.width(), 1);
        assert_eq!(grid.height(), 5);
    }

    #[test]
    fn test_grid_new_single_column() {
        let grid: Grid<u8> = Grid::new(8, 1);
        assert_eq!(grid.width(), 8);
        assert_eq!(grid.height(), 1);
    }

    #[test]
    fn test_grid_new_initializes_to_default() {
        let grid: Grid<i32> = Grid::new(2, 3);
        for i in 0..2 {
            for j in 0..3 {
                assert_eq!(*grid.value(i, j), 0, "Cell ({}, {}) should initialize to default", i, j);
            }
        }
    }

    #[test]
    fn test_grid_width_and_height() {
        let grid: Grid<String> = Grid::new(7, 11);
        assert_eq!(grid.width(), 7);
        assert_eq!(grid.height(), 11);
    }

    // ========== Value access via value() and value_mut() ==========

    #[test]
    fn test_value_read() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        *grid.value_mut(0, 0) = 42;
        *grid.value_mut(1, 1) = 99;

        assert_eq!(*grid.value(0, 0), 42);
        assert_eq!(*grid.value(1, 1), 99);
    }

    #[test]
    fn test_value_mut_write() {
        let mut grid: Grid<&str> = Grid::new(2, 2);
        *grid.value_mut(0, 1) = "hello";
        *grid.value_mut(1, 0) = "world";

        assert_eq!(*grid.value(0, 1), "hello");
        assert_eq!(*grid.value(1, 0), "world");
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_value_panics_row_out_of_bounds() {
        let grid: Grid<i32> = Grid::new(2, 2);
        let _ = grid.value(5, 0);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_value_panics_col_out_of_bounds() {
        let grid: Grid<i32> = Grid::new(2, 2);
        let _ = grid.value(0, 5);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_value_mut_panics_row_out_of_bounds() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        let _ = grid.value_mut(5, 0);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_value_mut_panics_col_out_of_bounds() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        let _ = grid.value_mut(0, 5);
    }

    // ========== Option-based access via get() and get_mut() ==========

    #[test]
    fn test_get_valid() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        *grid.value_mut(0, 0) = 55;

        let val = grid.get(0, 0);
        assert!(val.is_some());
        assert_eq!(*val.unwrap(), 55);
    }

    #[test]
    fn test_get_mut_valid() {
        let mut grid: Grid<i32> = Grid::new(2, 2);

        if let Some(cell) = grid.get_mut(1, 1) {
            *cell = 77;
        }

        assert_eq!(*grid.value(1, 1), 77);
    }

    #[test]
    fn test_get_returns_none_row_out_of_bounds() {
        let grid: Grid<i32> = Grid::new(2, 2);
        assert!(grid.get(5, 0).is_none());
    }

    #[test]
    fn test_get_returns_none_col_out_of_bounds() {
        let grid: Grid<i32> = Grid::new(2, 2);
        assert!(grid.get(0, 5).is_none());
    }

    #[test]
    fn test_get_mut_returns_none_row_out_of_bounds() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        assert!(grid.get_mut(5, 0).is_none());
    }

    #[test]
    fn test_get_mut_returns_none_col_out_of_bounds() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        assert!(grid.get_mut(0, 5).is_none());
    }

    // ========== Tuple index access via Index/IndexMut traits ==========

    #[test]
    fn test_index_trait() {
        let mut grid: Grid<i32> = Grid::new(2, 3);
        *grid.value_mut(0, 2) = 123;

        assert_eq!(grid[(0, 2)], 123);
    }

    #[test]
    fn test_index_mut_trait() {
        let mut grid: Grid<i32> = Grid::new(2, 3);
        grid[(1, 1)] = 456;

        assert_eq!(*grid.value(1, 1), 456);
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_index_trait_panics_out_of_bounds() {
        let grid: Grid<i32> = Grid::new(2, 2);
        let _ = grid[(5, 5)];
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_index_mut_trait_panics_out_of_bounds() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        grid[(5, 5)] = 1;
    }

    // ========== Row access and iteration ==========

    #[test]
    fn test_row_basic_access() {
        let mut grid: Grid<i32> = Grid::new(3, 3);
        *grid.value_mut(1, 0) = 10;
        *grid.value_mut(1, 1) = 20;
        *grid.value_mut(1, 2) = 30;

        let row = grid.row(1);
        assert_eq!(row[0], 10);
        assert_eq!(row[1], 20);
        assert_eq!(row[2], 30);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_row_panics_out_of_bounds() {
        let grid: Grid<i32> = Grid::new(2, 3);
        let _ = grid.row(5);
    }

    #[test]
    fn test_row_mut_modification() {
        let mut grid: Grid<i32> = Grid::new(2, 3);
        {
            let mut row = grid.row_mut(0);
            row[0] = 100;
            row[1] = 200;
        }

        assert_eq!(*grid.value(0, 0), 100);
        assert_eq!(*grid.value(0, 1), 200);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_row_mut_panics_out_of_bounds() {
        let mut grid: Grid<i32> = Grid::new(2, 3);
        let _ = grid.row_mut(5);
    }

    #[test]
    fn test_rows_returns_all_rows() {
        let mut grid: Grid<i32> = Grid::new(3, 2);
        for i in 0..3 {
            for j in 0..2 {
                *grid.value_mut(i, j) = (i * 10 + j) as i32;
            }
        }

        let rows = grid.rows();
        assert_eq!(rows.len(), 3);

        for i in 0..3 {
            for j in 0..2 {
                assert_eq!(rows[i][j], (i * 10 + j) as i32);
            }
        }
    }

    #[test]
    fn test_rows_empty_grid() {
        let grid: Grid<i32> = Grid::new(0, 5);
        let rows = grid.rows();
        assert_eq!(rows.len(), 0);
    }

    // ========== Column access and iteration ==========

    #[test]
    fn test_column_basic_access() {
        let mut grid: Grid<i32> = Grid::new(3, 2);
        *grid.value_mut(0, 1) = 11;
        *grid.value_mut(1, 1) = 22;
        *grid.value_mut(2, 1) = 33;

        let col = grid.column(1);
        assert_eq!(col[0], 11);
        assert_eq!(col[1], 22);
        assert_eq!(col[2], 33);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_column_panics_out_of_bounds() {
        let grid: Grid<i32> = Grid::new(2, 2);
        let _ = grid.column(5);
    }

    #[test]
    fn test_column_mut_modification() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        {
            let mut col = grid.column_mut(1);
            col[0] = 500;
            col[1] = 600;
        }

        assert_eq!(*grid.value(0, 1), 500);
        assert_eq!(*grid.value(1, 1), 600);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_column_mut_panics_out_of_bounds() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        let _ = grid.column_mut(5);
    }

    #[test]
    fn test_columns_returns_all_columns() {
        let mut grid: Grid<i32> = Grid::new(2, 3);
        for i in 0..2 {
            for j in 0..3 {
                *grid.value_mut(i, j) = (i * 10 + j) as i32;
            }
        }

        let columns = grid.columns();
        assert_eq!(columns.len(), 3);

        for i in 0..2 {
            for j in 0..3 {
                assert_eq!(columns[j][i], (i * 10 + j) as i32);
            }
        }
    }

    #[test]
    fn test_columns_empty_grid() {
        let grid: Grid<i32> = Grid::new(2, 0);
        let columns = grid.columns();
        assert_eq!(columns.len(), 0);
    }

    // ========== Unsafe get_unchecked methods ==========

    #[test]
    fn test_get_unchecked() {
        let mut grid: Grid<i32> = Grid::new(2, 2);
        *grid.value_mut(0, 0) = 777;

        unsafe {
            assert_eq!(*grid.get_unchecked(0, 0), 777);
        }
    }

    #[test]
    fn test_get_unchecked_mut() {
        let mut grid: Grid<i32> = Grid::new(2, 2);

        unsafe {
            *grid.get_unchecked_mut(1, 1) = 888;
        }

        assert_eq!(*grid.value(1, 1), 888);
    }

    // ========== Complex scenarios and edge cases ==========

    #[test]
    fn test_large_grid() {
        let mut grid: Grid<usize> = Grid::new(100, 100);

        for i in 0..100 {
            for j in 0..100 {
                *grid.value_mut(i, j) = i * 100 + j;
            }
        }

        // Spot check
        assert_eq!(*grid.value(50, 50), 50 * 100 + 50);
        assert_eq!(*grid.value(99, 99), 99 * 100 + 99);
    }

    #[test]
    fn test_alternating_access_and_mutation() {
        let mut grid: Grid<i32> = Grid::new(3, 3);

        // Set via value_mut
        *grid.value_mut(0, 0) = 1;

        // Read via value
        assert_eq!(*grid.value(0, 0), 1);

        // Read via get
        assert_eq!(*grid.get(0, 0).unwrap(), 1);

        // Mutate via get_mut
        *grid.get_mut(0, 0).unwrap() = 2;

        // Read via index
        assert_eq!(grid[(0, 0)], 2);

        // Mutate via index_mut
        grid[(0, 0)] = 3;

        // Final check
        assert_eq!(*grid.value(0, 0), 3);
    }

    #[test]
    fn test_grid_with_different_types() {
        let mut grid_str: Grid<&str> = Grid::new(2, 2);
        *grid_str.value_mut(0, 0) = "test";
        assert_eq!(*grid_str.value(0, 0), "test");

        let mut grid_f64: Grid<f64> = Grid::new(2, 2);
        *grid_f64.value_mut(1, 1) = 3.14;
        assert_eq!(*grid_f64.value(1, 1), 3.14);

        let mut grid_bool: Grid<bool> = Grid::new(2, 2);
        *grid_bool.value_mut(0, 1) = true;
        assert_eq!(*grid_bool.value(0, 1), true);
    }

    #[test]
    fn test_grid_multiple_swaps() {
        let mut grid: Grid<i32> = Grid::new(2, 3);
        for i in 0..2 {
            for j in 0..3 {
                *grid.value_mut(i, j) = (i * 10 + j) as i32;
            }
        }

        // Swap (0,0) with (0,1)
        grid.swap((0, 0), (0, 1));
        assert_eq!(*grid.value(0, 0), 1);
        assert_eq!(*grid.value(0, 1), 0);

        // Swap (0,1) with (1,1)
        grid.swap((0, 1), (1, 1));
        assert_eq!(*grid.value(0, 1), 11);
        assert_eq!(*grid.value(1, 1), 0);
    }

    #[test]
    fn test_default_trait() {
        let grid: Grid<i32> = Default::default();
        // Should compile and create a valid grid
        let _ = grid.width();
    }
}

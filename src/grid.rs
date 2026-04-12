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
}

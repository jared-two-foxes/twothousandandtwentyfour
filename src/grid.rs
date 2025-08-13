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

impl<T> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self
    where
        T: Default + Clone,
    {
        let mut data = Vec::new();
        data.resize(rows * cols, Default::default());
        Grid { data, rows, cols }
    }

    pub fn width(&self) -> usize {
        self.rows
    }

    pub fn height(&self) -> usize {
        self.cols
    }

    pub fn rows<'a>(&self) -> Vec<Row<'a, T>> {
        unimplemented!();
    }

    pub fn rows_mut<'a>(&self) -> Vec<RowMut<'a, T>> {
        unimplemented!();
    }

    pub fn columns<'a>(&self) -> Vec<Column<'a, T>> {
        unimplemented!()
    }

    pub fn column<'a>(&'a self, col: usize) -> Column<'a, T> {
        assert!(col < self.cols, "Index out of bounds");
        Column {
            grid: self,
            column: col
        }
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

    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &T {
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

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&T> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            unsafe { Some(self.get_unchecked_mut(row, col)) }
        }
    }

    pub fn swap(&mut self, (row_a, col_a): (usize, usize), (row_b, col_b): (usize, usize)) {
        //@todo: validate indices with an assert
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



struct Grid<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

struct GridRow<'a, T> {
    grid: &Grid<T>,
    row: usize,
    index: usize,
}

impl<'a, T> Iterator for GridRow<'a, T> {
    type Item = T;
  
    pub fn next(&mut self) -> Option<&Item> {
      if self.row > self.grid.rows() {
        return None;
      }
      let n = grid.get(self.row, self.index);
      self.index += 1;
      n
    }
}

struct GridColumn<'a> {
    grid: &Grid,
    column: usize,
}

impl<T> Grid<T> {
  pub fn new(rows: usize, cols: usize) -> Self 
  where T: Default {
      let mut data = Vec::new();
      data.resize(rows*cols);
      Grid {
        data,
        rows,
        cols
      }
  }
  
  pub fn rows(&self) -> Vec<GridRow> {
      unimplemented!();
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
        unsafe { Some(self.get_unchecked(row,col)) }
      }
    }
    
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&T> {
      if row >= self.rows || col >= self.cols {
        None
      } else {
        unsafe { Some(self.get_unchecked_mut(row,col)) }
      }
    }
    
    pub fn swap(&self, (row_a, col_a): (usize, usize), (row_b, col_b): (usize, usize)) {
      //@todo: validate indices with an assert
        let l = self.get_index(row_a, col_a);
        let r = self.to_index(row_b, col_b);
        self.data.swap(l,r);
    }
}

/*
impl<T> Index<(usize, usize)> for Grid<T> {
  type Output = T;
  #[inline]
  fn index(&self, (row,col): (usize, usize)) -> &T {
    assert!(row < self.rows && col < self.cols, "");
    let index = self.get_index(row,col);
    &self.data[index]
  }
}

impl<T> IndexMut<(usize,usize)> 
for Grid<T> {
  type Output = T;
  #[inline]
   fn index_mut(&self, (row,col): (usize, usize)) -> &mut T {
    assert!(
      row < self.rows && col < self.cols, 
      "");
    let index = self.get_index(row,col);
    &mut self.data[index]
  }
}
*/
// A 2-dimensional distinct coordinate
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
  #[inline(always)]
  #[must_use]
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }
  
  #[inline]
  #[must_use]
  pub fn splat(v: i32) -> Self {
    Self { x: v, y: v }
  }
}

impl Add<Vec2> for Vec2 {
  type Output = Self;
  #[inline]
  pub fn add(self, rhs: Self) -> Self {
    Vec2 {
      x: self.x.add(rhs.x),
      y: self.y.add(rhs.y)
    } 
  }
}

impl Add<&Vec2> for Vec2 {
  type Output = Vec2;
  #[inline]
  pub fn add(self, rhs: &Vec2) -> Vec2 {
    self.add(*rhs)
  }
}

impl Add<&Vec2> for &Vec2 {
  type Output = Vec2;
  #[inline]
  pub fn add(&self, rhs: &Vec2) -> Vec2 {
    (*self).add(*rhs)
  }
}

impl Add<Vec2> for &Vec2 {
  type Output = Vec2;
  #[inline]
  pub fn add(&self, rhs: Vec2) -> Vec2 {
    (*self).add(rhs)
  }
}

impl AddAssign<Vec2> for Vec2 {
  #[inline]
  pub fn add_assign(&mut self, rhs: Vec2) {
    self.x.add_assign(rhs.x);
    self.y.add_assign(rhs.y);
  }
}

impl AddAssign<&Vec2> for Vec2 {
  #[inline]
  pub fn add_assign(&mut self, rhs: &Vec2) {
    self.add_assign(*rhs)
  }
}
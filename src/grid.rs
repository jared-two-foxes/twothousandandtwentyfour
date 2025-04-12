use crate::Vec2;

#[derive (Default)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn T& at(p: &Vec2) -> Option<&T> {
        let i = self.to_index(p);
        self.data[i]
    }
    
    fn to_index(&self, p: &Vec2) -> usize {
        (p.y * self.width + p.x).into()
    }
    
    fn swap(&self, a: &Vec2, b: &Vec2) {
        let l = self.to_index(a);
        let r = self.to_index(b);
        data.swap(l,r);
    }
}
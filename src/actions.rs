use crate::{
    model::Model,
    vec2::Vec2,
}

enum Direction {
  Left,
  Right,
  Up,
  Down,
}

enum Message {
    Compress(Direction), 
}

fn to_start(dir: Direction) -> Vec2 {
  match dir {
    Left => Vec2::new(0, 0),
    Right => Vec2::new(3, 3),
    Up => Vec2::new(0, 0),
    Down => Vec::new(3, 3),
  }
}

fn to_stride(dir: Direction) -> Vec2 {
  match dir {
    Left => Vec2::new(-1, 0),
    Right => Vec2::new(1, 0),
    Up => Vec2::new(0, 1),
    Down => Vec2::new(0, -1),
  }
}

fn update(message: Message, model: &mut Model) -> Option<Message> {
  match message {
    Message::Compress(dir) => {
       let start = to_start(dir);
       model.score += compress(
         model.grid, 
         start, 
         dir).unwrap_or(0);
       None
    },
    _ => None,  
  }
} 

// going to start on the first square (i, with value x) on the left, 
    // compare it with the next square to its right (j); if we hit an 
    // empty space we'll try the next square (j+1), if its got the same 
    // value as us then combine with us and increment value set j+1 to 
    // empty then increment j, some value not x and x is empty then swap
    // with i increment i and repeat.
pub fn compress(grid: &mut Grid<u16>, start: Vec2, stride: Vec2) -> Result<u32> {
    let mut score = 0;
    let mut i = start;
    let mut j = i + stride;
    while let Some(v) = grid.at(i) {
        match grid.at(j) {
            Some(sv) => { 
                if v == 0 && sv != 0 {
                    grid.swap(i,j);
                    i += stride;
                } else if sv == v { 
                    *v *= 2;
                    score += *v;
                    *sv = 0;
                    i += stride;
                } 
                j += stride;
            }, 
            None => {  
                break;
            },
        }
    }
    Ok(score)
} 

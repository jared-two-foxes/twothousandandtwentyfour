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

pub fn update(message: Message, model: &mut Model) -> Option<Message> {
  match message {
    Message::Compress(dir) => {
        let (rows, score) = model.grid
            .rows()
            .iter()
            .fold((Vec::new(), 0), |(list, s), row| {
                let (new_row, score) = compress_left(row.iter());
                list.push(new_row);
                (list, s + score)
            });
        model.grid = Grid::from_rows(rows); 
        model.score += score;
        
        if highest_tile == 11 {
            // player won!  
            // transition game state?
        }
        
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
fn compress_left<T>(iter: impl IntoIterator<Item=T>) -> (Vec<T>, u32) {
    let mut score = 0;
    let mut stack = Vec::with_capacity(4);
    while let Some(v) = iter.find(|&&x| x != 0) {
        if let Some(n) = stack.last_mut() {
            if n == v {
                *n += 1;
                score += *n;
            } else {
                stack.push(*v);
            }
        } else {
            stack.push(*v)
        }
    }
    stack.resize(4, 0);
    (stack, score)
} 

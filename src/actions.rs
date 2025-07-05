use crate::{
    model::Model,
    vec2::Vec2,
}

pub enum Direction {
  Left,
  Right,
  Up,
  Down,
}

pub enum Message {
    Compress(Direction), 
}

pub fn update(model: &mut Model, message: Message) -> Option<Message> {
  match message {
    Message::Compress(dir) => {
        match dir {
            Direction::Left => compress_left(model),
            Direction::Right => compress_right(model),
            Direction::Up => compress_up(model),
            Direction::Down => compress_down(model)
        }
        
        if highest_tile(model) == 11 {
            // player won!  
            // transition game state?
        }
        
        None
    },
    _ => None,  
  }
} 

fn compress_left(model: &mut Model) {
    let (rows, score) = model.grid
        .rows()
        .iter()
        .fold((Vec::new(), 0), |(list, s), row| {
            let (new_row, score) = compress(row, back_inserter_policy);
            list.push(new_row);
            (list, s + score)
        });
        
    model.update(
        Grid::from_rows(rows),
        score);
}

// going to start on the first square (i, with value x) on the left, 
// compare it with the next square to its right (j); if we hit an 
// empty space we'll try the next square (j+1), if its got the same 
// value as us then combine with us and increment value set j+1 to 
// empty then increment j, some value not x and x is empty then swap
// with i increment i and repeat.
fn compress<T>(collection: impl IntoIterator<Item=T>, inserter: InserterPolicy) -> u32 {
    let mut score = 0;
    let stack = Vec::with_capacity(4);
    while let Some(v) = collection.into_iter().find(|&&x| x != 0) {
        if let Some(n) = stack.last_mut() {
            if n == v {
                *n += 1;
                score += *n;
            } else {
                inserter(stack, *v);
            }
        } else {
            inserter(stack, *v);
        }
    }
    stack.resize(4, 0);
    (stack, score)
} 

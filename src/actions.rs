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
        // else if no available moves remain?
        
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

fn next_left(row: &[u16], i: usize) -> Option<usise> {
    while i < row.len() {
        if row[i] != 0 {
            return Some(i);
        } else {
            i += 1;
        }
    }
    None
}

fn next_right(row: &[u16], i: isize) -> Option<isize> {

}

fn compress_left(row: &mut [u16]) {
    let mut i = 0;
    let mut j = 1;
    while i < row.len() {
        match next_valid(row, j) {
            Some(x) => {
                if row[i] != 0 {
                    if row[i] == row[x] {
                        row[i] += 1;
                        row[x] = 0;
                        j = x + 1;
                    }
                    i += 1;
                } else {
                    row[i] = row[x];
                    row[x] = 0;  
                    j = x + 1;
                }
            },
            None => { break; },
        }
    }
}

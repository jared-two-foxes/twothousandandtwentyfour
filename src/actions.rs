use crate::model::Model;

use std::ops::{Index, IndexMut};

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
                Direction::Down => compress_down(model),
            }

            if highest_tile(model) == 11 {
                // player won!
                // transition game state?
            }
            // else if no available moves remain?

            None
        }
    }
}

fn highest_tile(model: &Model) -> u16 {
    model.grid.data.iter()
        .cloned()
        .max()
        .unwrap_or(0)
}

fn compress_left(model: &mut Model) {
    let len = model.grid.width();
    for i in 0..model.grid.height() {
        compress_row_left(&mut model.grid.row_mut(i), len);
    }
}

fn compress_right(model: &mut Model) {
    let len = model.grid.width();
    for i in 0..model.grid.height() {
        compress_row_right(&mut model.grid.row_mut(i), len);
    }
}

fn compress_up(model: &mut Model) {
    let len = model.grid.height();
    for i in 0..model.grid.width() {
        compress_row_left(&mut model.grid.column_mut(i), len);
    }
}

fn compress_down(model: &mut Model) {
    let len = model.grid.height();
    for i in 0..model.grid.width() {
        compress_row_right(&mut model.grid.column_mut(i), len);
    }
}

fn next_left<T>(row: &T, mut i: usize, n: usize) -> Option<usize>
where
    T: Index<usize, Output = u16>,
{
    while i < n {
        if row[i] != 0 {
            return Some(i);
        } else {
            i += 1;
        }
    }
    None
}

fn compress_row_left<T>(row: &mut T, n: usize)
where
    T: IndexMut<usize, Output = u16>,
{
    let mut i = 0;
    let mut j = 1;
    while i < n {
        match next_left(row, j, n) {
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
            }
            None => {
                break;
            }
        }
    }
}

fn next_right<T>(_row: &T, mut _i: usize, _n: usize) -> Option<usize>
where
    T: Index<usize, Output = u16>,
{
    unimplemented!()
}

fn compress_row_right<T>(_row: &mut T, _n: usize)
where
    T: IndexMut<usize, Output = u16>,
{
    unimplemented!()
}

#[cfg(test)]
mod tests {
#[test]
fn test_compress_left() {
    let mut model = Model::new(4, 4); // Assuming the grid size is 4x4
    
    // Example grid:
    // [2, 2, 4, 0]
    // [0, 2, 2, 2]
    // [0, 0, 0, 4]
    // [0, 0, 0, 0]
    model.grid.set_row(0, &[2, 2, 4, 0]);
    model.grid.set_row(1, &[0, 2, 2, 2]);
    model.grid.set_row(2, &[0, 0, 0, 4]);
    model.grid.set_row(3, &[0, 0, 0, 0]);

    compress_left(&mut model);

    // Expected output:
    // [4, 4, 0, 0]
    // [4, 2, 0, 0]
    // [4, 0, 0, 0]
    // [0, 0, 0, 0]
    assert_eq!(model.grid.get_row(0), &[4, 4, 0, 0]);
    assert_eq!(model.grid.get_row(1), &[4, 2, 0, 0]);
    assert_eq!(model.grid.get_row(2), &[4, 0, 0, 0]);
    assert_eq!(model.grid.get_row(3), &[0, 0, 0, 0]);
}

#[test]
fn test_compress_left_empty_rows() {
    let mut model = Model::new(4, 4); // Grid size 4x4
    
    // Empty grid:
    // [0, 0, 0, 0]
    // [0, 0, 0, 0]
    // [0, 0, 0, 0]
    // [0, 0, 0, 0]
    for i in 0..4 {
        model.grid.set_row(i, &[0, 0, 0, 0]);
    }

    compress_left(&mut model);

    // Expected output: Should remain the same as the input
    for i in 0..4 {
        assert_eq!(model.grid.get_row(i), &[0, 0, 0, 0]);
    }
}

#[test]
fn test_compress_up() {
    let mut model = Model::new(4, 4);
    
    // Example grid:
    // [2, 0, 0, 0]
    // [2, 2, 4, 4]
    // [4, 4, 0, 0]
    // [0, 4, 0, 4]
    model.grid.set_row(0, &[2, 0, 0, 0]);
    model.grid.set_row(1, &[2, 2, 4, 4]);
    model.grid.set_row(2, &[4, 4, 0, 0]);
    model.grid.set_row(3, &[0, 4, 0, 4]);

    compress_up(&mut model);

    // Expected output:
    // [4, 2, 4, 8]
    // [4, 8, 0, 0]
    // [0, 0, 0, 0]
    // [0, 0, 0, 0]
    assert_eq!(model.grid.get_row(0), &[4, 2, 4, 8]);
    assert_eq!(model.grid.get_row(1), &[4, 8, 0, 0]);
    assert_eq!(model.grid.get_row(2), &[0, 0, 0, 0]);
    assert_eq!(model.grid.get_row(3), &[0, 0, 0, 0]);
}

}

use crate::model::Model;
//use crate::grid::Grid;

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

fn highest_tile(model: &Model) -> u8 {
    0
}

fn compress_left(model: &mut Model) {
    let len = model.grid.width();
    let rows = model.grid.rows_mut().iter_mut().fold(Vec::new(), |mut list, mut row| {
        let new_row = compress_row_left(row, len);
        list.push(new_row);
        list
    });

    //model.update(Grid::from_rows(rows), score);
}

fn compress_right(model: &mut Model) {
    let len = model.grid.width();
    let rows = model.grid.rows_mut().iter_mut().fold(Vec::new(), |mut list, row| {
        let new_row = compress_row_right(row, len);
        list.push(new_row);
        list
    });

    //model.update(Grid::from_rows(rows), score);
}

fn compress_up(model: &mut Model) {
    unimplemented!();
}

fn compress_down(model: &Model) {
    unimplemented!();
}

fn next_left<T>(row: &T, mut i: usize, n: usize) -> Option<usize>
where
    T: Index<usize, Output = u16>
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
    T: IndexMut<usize, Output = u16>
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

fn next_right<T>(row: &T, mut i: usize, n: usize) -> Option<usize>
where
    T: Index<usize, Output = u16>
{
    None
}

fn compress_row_right<T>(row: &mut T, n: usize)
where
    T: IndexMut<usize, Output = u16>
{
    unimplemented!()
}

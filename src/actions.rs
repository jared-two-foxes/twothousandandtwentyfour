use crate::model::{Model, State};

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
            let value = match dir {
                Direction::Left => compress_left(model),
                Direction::Right => compress_right(model),
                Direction::Up => compress_up(model),
                Direction::Down => compress_down(model),
            };

            model.score += value as u32;

            // Fill a new square
            model.generate_new_value();

            if highest_tile(model) == 11 || model.check_for_valid_moves() {
                model.state = State::Done;
            }

            None
        }
    }
}

fn highest_tile(model: &Model) -> u16 {
    let mut max_val = 0u16;
    for i in 0..model.grid.height() {
        for j in 0..model.grid.width() {
            if let Some(&val) = model.grid.get(i, j) {
                max_val = max_val.max(val);
            }
        }
    }
    max_val
}

fn compress_left(model: &mut Model) -> u16 {
    let mut result = 0;
    let len = model.grid.width();
    for i in 0..model.grid.height() {
        result += compress_row_left(&mut model.grid.row_mut(i), len);
    }
    result
}

fn compress_right(model: &mut Model) -> u16 {
    let mut result = 0;
    let len = model.grid.width();
    for i in 0..model.grid.height() {
        result += compress_row_right(&mut model.grid.row_mut(i), len as isize);
    }
    result
}

fn compress_up(model: &mut Model) -> u16 {
    let mut result = 0;
    let len = model.grid.height();
    for i in 0..model.grid.width() {
        result += compress_row_left(&mut model.grid.column_mut(i), len);
    }
    result
}

fn compress_down(model: &mut Model) -> u16 {
    let mut result = 0;
    let len = model.grid.height();
    for i in 0..model.grid.width() {
        result += compress_row_right(&mut model.grid.column_mut(i), len as isize);
    }
    result
}

fn next_right<T>(row: &T, mut i: usize, n: usize) -> Option<usize>
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

fn compress_row_left<T>(row: &mut T, n: usize) -> T::Output
where
    T: IndexMut<usize, Output = u16>,
{
    let mut i = 0;
    let mut j = 1;
    let mut v = 0;
    while i < n {
        match next_right(row, j, n) {
            Some(x) => {
                if row[i] != 0 {
                    if row[i] == row[x] {
                        row[i] += 1;
                        row[x] = 0;
                        j = x + 1;
                        v += row[i];
                    }
                    i += 1;
                    j = usize::max(j, i + 1);
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
    v
}

fn next_left<T>(row: &T, mut i: isize) -> Option<usize>
where
    T: Index<usize, Output = u16>,
{
    while i >= 0 {
        if row[i as usize] != 0 {
            return Some(i as usize);
        } else {
            i -= 1;
        }
    }
    None
}

fn compress_row_right<T>(row: &mut T, n: isize) -> T::Output
where
    T: IndexMut<usize, Output = u16>,
{
    let mut i = n - 1;
    let mut j = i - 1;
    let mut v = 0;
    while i >= 0 {
        match next_left(row, j) {
            Some(x) => {
                if row[i as usize] != 0 {
                    if row[i as usize] == row[x] {
                        row[i as usize] += 1;
                        row[x] = 0;
                        j = x as isize - 1;
                        v += row[i as usize];
                    }
                    i -= 1;
                    j = isize::min(j, i - 1);
                } else {
                    row[i as usize] = row[x];
                    row[x] = 0;
                    j = x as isize - 1;
                }
            }
            None => {
                break;
            }
        }
    }
    v
}

// TODO: Tests are currently disabled due to missing Grid::set_row/get_row methods.
// These will be rebuilt in backlog item 9 (Rebuild Test Coverage).
// #[cfg(test)]
// mod tests {
//     use super::*;
// }

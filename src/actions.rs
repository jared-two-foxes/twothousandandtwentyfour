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

            model.score += value;

            // Fill a new square
            model.generate_new_value();

            if highest_tile(model) == 11 || !model.check_for_valid_moves() {
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

fn compress_left(model: &mut Model) -> u32 {
    let mut result = 0;
    let len = model.grid.width();
    for i in 0..model.grid.height() {
        result += compress_row_left(&mut model.grid.row_mut(i), len);
    }
    result
}

fn compress_right(model: &mut Model) -> u32 {
    let mut result = 0;
    let len = model.grid.width();
    for i in 0..model.grid.height() {
        result += compress_row_right(&mut model.grid.row_mut(i), len as isize);
    }
    result
}

fn compress_up(model: &mut Model) -> u32 {
    let mut result = 0;
    let len = model.grid.height();
    for i in 0..model.grid.width() {
        result += compress_row_left(&mut model.grid.column_mut(i), len);
    }
    result
}

fn compress_down(model: &mut Model) -> u32 {
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

fn compress_row_left<T>(row: &mut T, n: usize) -> u32
where
    T: IndexMut<usize, Output = u16>,
{
    let mut i = 0;
    let mut j = 1;
    let mut v: u32 = 0;
    while i < n {
        match next_right(row, j, n) {
            Some(x) => {
                if row[i] != 0 {
                    if row[i] == row[x] {
                        row[i] += 1;
                        row[x] = 0;
                        j = x + 1;
                        v += 2u32.pow(u32::from(row[i]));
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

fn compress_row_right<T>(row: &mut T, n: isize) -> u32
where
    T: IndexMut<usize, Output = u16>,
{
    let mut i = n - 1;
    let mut j = i - 1;
    let mut v: u32 = 0;
    while i >= 0 {
        match next_left(row, j) {
            Some(x) => {
                if row[i as usize] != 0 {
                    if row[i as usize] == row[x] {
                        row[i as usize] += 1;
                        row[x] = 0;
                        j = x as isize - 1;
                        v += 2u32.pow(u32::from(row[i as usize]));
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

#[cfg(test)]
mod tests {
    use super::*;

    fn clear_grid(model: &mut Model) {
        for i in 0..model.grid.height() {
            for j in 0..model.grid.width() {
                *model.grid.value_mut(i, j) = 0;
            }
        }
    }

    fn set_grid(model: &mut Model, values: [[u16; 4]; 4]) {
        for (i, row) in values.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                *model.grid.value_mut(i, j) = *value;
            }
        }
    }

    #[test]
    fn compress_row_left_merges_exponents() {
        let mut row = vec![1u16, 1u16, 0u16, 0u16];
        let merged = compress_row_left(&mut row, 4);

        assert_eq!(row, vec![2u16, 0u16, 0u16, 0u16]);
        assert_eq!(merged, 4u32);
    }

    #[test]
    fn compress_row_left_merges_once_per_pair() {
        let mut row = vec![2u16, 2u16, 2u16, 0u16];
        let merged = compress_row_left(&mut row, 4);

        assert_eq!(row, vec![3u16, 2u16, 0u16, 0u16]);
        assert_eq!(merged, 8u32);
    }

    #[test]
    fn compress_row_left_scores_multiple_merges_by_display_value() {
        let mut row = vec![1u16, 1u16, 1u16, 1u16];
        let merged = compress_row_left(&mut row, 4);

        assert_eq!(row, vec![2u16, 2u16, 0u16, 0u16]);
        assert_eq!(merged, 8u32);
    }

    #[test]
    fn compress_left_keeps_grid_in_exponent_domain() {
        let mut model = Model::new();
        clear_grid(&mut model);

        *model.grid.value_mut(0, 0) = 1;
        *model.grid.value_mut(0, 1) = 1;
        *model.grid.value_mut(1, 0) = 2;
        *model.grid.value_mut(1, 1) = 2;

        let _ = compress_left(&mut model);

        assert_eq!(*model.grid.value(0, 0), 2);
        assert_eq!(*model.grid.value(0, 1), 0);
        assert_eq!(*model.grid.value(1, 0), 3);
        assert_eq!(*model.grid.value(1, 1), 0);

        for i in 0..model.grid.height() {
            for j in 0..model.grid.width() {
                let value = *model.grid.value(i, j);
                assert!(value <= 11);
            }
        }
    }

    #[test]
    fn update_keeps_running_when_moves_remain() {
        let mut model = Model::new();
        clear_grid(&mut model);
        set_grid(
            &mut model,
            [
                [1, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        );

        let _ = update(&mut model, Message::Compress(Direction::Left));

        assert!(matches!(model.state, State::Running));
    }

    #[test]
    fn update_sets_done_when_2048_reached() {
        let mut model = Model::new();
        clear_grid(&mut model);
        set_grid(
            &mut model,
            [
                [10, 10, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        );

        let _ = update(&mut model, Message::Compress(Direction::Left));

        assert!(matches!(model.state, State::Done));
    }

    #[test]
    fn update_sets_done_when_no_valid_moves_remain() {
        let mut model = Model::new();
        clear_grid(&mut model);
        set_grid(
            &mut model,
            [
                [1, 2, 3, 4],
                [2, 3, 4, 1],
                [3, 4, 1, 2],
                [4, 1, 2, 3],
            ],
        );

        let _ = update(&mut model, Message::Compress(Direction::Left));

        assert!(matches!(model.state, State::Done));
    }
}

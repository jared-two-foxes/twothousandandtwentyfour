use twentyfourtyeight::actions::{self, Direction, Message};
use twentyfourtyeight::model::{Model, State};

fn clear_grid(model: &mut Model) {
    for row in 0..model.grid.height() {
        for col in 0..model.grid.width() {
            *model.grid.value_mut(row, col) = 0;
        }
    }
}

fn set_grid(model: &mut Model, values: [[u16; 4]; 4]) {
    for (row, row_values) in values.iter().enumerate() {
        for (col, value) in row_values.iter().enumerate() {
            *model.grid.value_mut(row, col) = *value;
        }
    }
}

fn non_zero_tiles(model: &Model) -> Vec<u16> {
    let mut values = Vec::new();
    for row in 0..model.grid.height() {
        for col in 0..model.grid.width() {
            let v = *model.grid.value(row, col);
            if v != 0 {
                values.push(v);
            }
        }
    }
    values
}

#[test]
fn move_updates_board_score_and_spawns_a_tile() {
    let mut model = Model::new();
    clear_grid(&mut model);
    set_grid(
        &mut model,
        [
            [1, 1, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ],
    );

    let _ = actions::update(&mut model, Message::Compress(Direction::Left));

    // The merge result is deterministic; only the newly spawned tile location/value is random.
    assert_eq!(*model.grid.value(0, 0), 2);
    assert_eq!(model.score, 4);
    assert!(matches!(model.state, State::Running));

    let values = non_zero_tiles(&model);
    assert_eq!(values.len(), 2);
    assert!(values.iter().all(|v| *v == 1 || *v == 2));
}

#[test]
fn reaching_2048_sets_won_state() {
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

    let _ = actions::update(&mut model, Message::Compress(Direction::Left));

    assert!(matches!(model.state, State::Won));
    assert_eq!(model.score, 2048);
    assert_eq!(*model.grid.value(0, 0), 11);
}

#[test]
fn no_valid_moves_sets_lost_state() {
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

    let _ = actions::update(&mut model, Message::Compress(Direction::Left));

    assert!(matches!(model.state, State::Lost));
    assert_eq!(model.score, 0);
}

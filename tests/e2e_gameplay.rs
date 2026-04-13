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

// 8.2: second move in the same direction is a no-op — board and score unchanged
#[test]
fn second_move_same_direction_is_no_op() {
    // Tiles are already left-aligned; both presses of Left are guaranteed no-ops
    // so no tile should ever be spawned.
    let mut model = Model::new();
    clear_grid(&mut model);
    set_grid(
        &mut model,
        [
            [1, 0, 0, 0],
            [2, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ],
    );

    let snapshot_before: Vec<u16> = (0..4)
        .flat_map(|i| (0..4).map(move |j| (i, j)))
        .map(|(i, j)| *model.grid.value(i, j))
        .collect();

    let _ = actions::update(&mut model, Message::Compress(Direction::Left));
    let _ = actions::update(&mut model, Message::Compress(Direction::Left));

    let snapshot_after: Vec<u16> = (0..4)
        .flat_map(|i| (0..4).map(move |j| (i, j)))
        .map(|(i, j)| *model.grid.value(i, j))
        .collect();

    assert_eq!(snapshot_before, snapshot_after);
    assert_eq!(model.score, 0);
}

// 8.6: score uses display value (2^exponent), not the raw exponent
#[test]
fn merge_score_is_display_value_not_exponent() {
    // Two exponent-2 tiles (display value 4 each) merge into exponent-3 (display value 8).
    // Score should increase by 8, not by 3.
    let mut model = Model::new();
    clear_grid(&mut model);
    set_grid(
        &mut model,
        [
            [2, 2, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ],
    );

    let _ = actions::update(&mut model, Message::Compress(Direction::Left));

    assert_eq!(*model.grid.value(0, 0), 3); // 2^3 = 8
    assert_eq!(model.score, 8);
}

// 8.7: full state machine — Running → Won → WonContinue
#[test]
fn state_transitions_running_won_won_continue() {
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

    assert!(matches!(model.state, State::Running));

    let _ = actions::update(&mut model, Message::Compress(Direction::Left));
    assert!(matches!(model.state, State::Won));

    // Simulate the player choosing to continue.
    model.state = State::WonContinue;
    assert!(matches!(model.state, State::WonContinue));

    // Further moves do not revert back to Won (guard in update).
    let _ = actions::update(&mut model, Message::Compress(Direction::Left));
    assert!(!matches!(model.state, State::Won));
}

// 8.7 (cont.): Running → Lost
#[test]
fn state_transitions_running_to_lost() {
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

    assert!(matches!(model.state, State::Running));

    let _ = actions::update(&mut model, Message::Compress(Direction::Left));
    assert!(matches!(model.state, State::Lost));
}

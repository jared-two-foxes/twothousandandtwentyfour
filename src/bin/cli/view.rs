use twentyfourtyeight::{grid::Grid, model::Model, model::State};

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn view(model: &Model, high_score: u32, frame: &mut Frame) {
    if let Some(text) = state_overlay_text(model.state) {
        draw_overlay(frame, text);
        return;
    }

    let vertical_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ]);
    let [header, row1, row2, row3, row4, footer] = vertical_layout.areas(frame.area());

    let score_text = format!("Score: {}    High Score: {}", model.score, high_score);
    frame.render_widget(Paragraph::new(score_text), header);

    draw_row(frame, row1, 0, &model.grid);
    draw_row(frame, row2, 1, &model.grid);
    draw_row(frame, row3, 2, &model.grid);
    draw_row(frame, row4, 3, &model.grid);

    frame.render_widget(
        Paragraph::new("\u{2191}\u{2193}\u{2190}\u{2192} Move | [Q]uit | [C]ontinue (after win) | [R]estart"),
        footer,
    );
}

fn state_overlay_text(state: State) -> Option<&'static str> {
    match state {
        State::Won => Some("You reached 2048! [C]ontinue or [Q]uit or [R]estart?"),
        State::Lost => Some("Game Over. No moves left. [R]estart or [Q]uit?"),
        State::Running | State::WonContinue => None,
    }
}

fn draw_overlay(frame: &mut Frame, text: &str) {
    let block = Block::default().borders(Borders::ALL).title("2048");
    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, frame.area());
}

fn draw_row(frame: &mut Frame, area: Rect, i: usize, grid: &Grid<u16>) {
    let layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);
    // let areas = layout.areas(area);
    // for j in 0..4 {
    for (j, area) in layout.areas::<4>(area).iter().enumerate() {
        let tile_text = display_tile_value(*grid.get(i, j).unwrap());
        frame.render_widget(Paragraph::new(tile_text), *area);
    }
}

fn display_tile_value(exponent: u16) -> String {
    if exponent == 0 {
        String::new()
    } else {
        2u32.pow(u32::from(exponent)).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{display_tile_value, state_overlay_text};
    use twentyfourtyeight::model::State;

    #[test]
    fn renders_empty_tile_as_blank() {
        assert_eq!(display_tile_value(0), "");
    }

    #[test]
    fn renders_exponents_as_powers_of_two() {
        assert_eq!(display_tile_value(1), "2");
        assert_eq!(display_tile_value(2), "4");
        assert_eq!(display_tile_value(11), "2048");
    }

    #[test]
    fn overlay_text_for_won_and_lost_states() {
        let won = state_overlay_text(State::Won);
        let lost = state_overlay_text(State::Lost);

        assert!(won.is_some());
        assert!(lost.is_some());
    }

    #[test]
    fn overlay_hidden_for_running_and_continue_states() {
        assert!(state_overlay_text(State::Running).is_none());
        assert!(state_overlay_text(State::WonContinue).is_none());
    }
}

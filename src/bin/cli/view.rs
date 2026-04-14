use twentyfourtyeight::{grid::Grid, model::Model, model::State};

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn view(model: &Model, high_score: u32, frame: &mut Frame) {
    if let Some(text) = state_overlay_text(model.state) {
        draw_overlay(frame, text);
        return;
    }

    let vertical_layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(2),
    ]);
    let [header, board_area, footer] = vertical_layout.areas(frame.area());

    let score_text = format!("Score: {}    High Score: {}", model.score, high_score);
    frame.render_widget(Paragraph::new(score_text).alignment(Alignment::Center), header);

    draw_board(frame, board_area, &model.grid);

    frame.render_widget(
        Paragraph::new("\u{2191}\u{2193}\u{2190}\u{2192} Move | [Q]uit | [C]ontinue (after win) | [R]estart")
            .alignment(Alignment::Center),
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

fn draw_board(frame: &mut Frame, area: Rect, grid: &Grid<u16>) {
    let board_block = Block::default()
        .borders(Borders::ALL)
        .title(" Board ")
        .style(Style::default().fg(Color::Gray));
    let inner = board_block.inner(area);
    frame.render_widget(board_block, area);

    if inner.width < 4 || inner.height < 4 {
        return;
    }

    if inner.width < 16 || inner.height < 8 {
        draw_compact_board(frame, inner, grid);
        return;
    }

    let mut tile_gap = 1;
    let mut tile_size = max_square_tile_size(inner, tile_gap);
    let mut board_size = tile_size.saturating_mul(4) + tile_gap.saturating_mul(3);

    if board_size > inner.width || board_size > inner.height {
        tile_gap = 0;
        tile_size = max_square_tile_size(inner, tile_gap);
        board_size = tile_size.saturating_mul(4);
    }

    if tile_size < 3 || board_size > inner.width || board_size > inner.height {
        draw_compact_board(frame, inner, grid);
        return;
    }

    let board_x = inner.x + inner.width.saturating_sub(board_size) / 2;
    let board_y = inner.y + inner.height.saturating_sub(board_size) / 2;

    for row in 0..4 {
        for col in 0..4 {
            let tile_rect = Rect::new(
                board_x + col as u16 * (tile_size + tile_gap),
                board_y + row as u16 * (tile_size + tile_gap),
                tile_size,
                tile_size,
            );
            draw_tile(frame, tile_rect, *grid.get(row, col).unwrap());
        }
    }
}

fn max_square_tile_size(area: Rect, gap: u16) -> u16 {
    let horizontal = area.width.saturating_sub(gap.saturating_mul(3)) / 4;
    let vertical = area.height.saturating_sub(gap.saturating_mul(3)) / 4;
    horizontal.min(vertical).max(1)
}

fn draw_tile(frame: &mut Frame, area: Rect, exponent: u16) {
    let value = display_tile_value(exponent);
    let style = tile_style(exponent);

    let block = Block::default().borders(Borders::ALL).style(style);
    frame.render_widget(block, area);

    let content = Block::default().borders(Borders::ALL).inner(area);
    if content.width == 0 || content.height == 0 {
        return;
    }

    frame.render_widget(
        Paragraph::new(value).alignment(Alignment::Center).style(style),
        content,
    );
}

fn draw_compact_board(frame: &mut Frame, area: Rect, grid: &Grid<u16>) {
    let row_constraints = [
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ];
    let rows = Layout::vertical(row_constraints).split(area);

    for i in 0..4 {
        if i >= rows.len() {
            break;
        }

        let mut row_text = String::new();
        for j in 0..4 {
            let exponent = *grid.get(i, j).unwrap();
            let value = display_tile_value(exponent);
            let cell = if value.is_empty() { "." } else { value.as_str() };
            if j > 0 {
                row_text.push(' ');
            }
            row_text.push_str(&format!("{:>4}", cell));
        }

        frame.render_widget(Paragraph::new(row_text).alignment(Alignment::Center), rows[i]);
    }
}

fn tile_style(exponent: u16) -> Style {
    let fg = match exponent {
        0  => Color::DarkGray,
        1  => Color::White,             // 2
        2  => Color::Cyan,              // 4
        3  => Color::LightCyan,         // 8
        4  => Color::Blue,              // 16
        5  => Color::LightBlue,         // 32
        6  => Color::Green,             // 64
        7  => Color::LightGreen,        // 128
        8  => Color::Magenta,           // 256
        9  => Color::LightMagenta,      // 512
        10 => Color::Yellow,            // 1024
        _  => Color::Red,               // 2048+
    };

    let mut style = Style::default().fg(fg);
    if exponent != 0 {
        style = style.add_modifier(Modifier::BOLD);
    } else {
        style = style.add_modifier(Modifier::DIM);
    }
    style
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
    use super::{display_tile_value, state_overlay_text, tile_style};
    use ratatui::style::Color;
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

    #[test]
    fn tile_colors_vary_by_exponent() {
        // Empty tile should be dim.
        assert_eq!(tile_style(0).fg, Some(Color::DarkGray));
        // 2048 (exponent 11) should be red.
        assert_eq!(tile_style(11).fg, Some(Color::Red));
        // 1024 (exponent 10) should be yellow.
        assert_eq!(tile_style(10).fg, Some(Color::Yellow));
        // Lower tiles should differ from 2048.
        assert_ne!(tile_style(1).fg, tile_style(11).fg);
    }
}

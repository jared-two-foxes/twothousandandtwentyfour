use twentyfourtyeight::{grid::Grid, model::Model};

use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub fn view(model: &Model, frame: &mut Frame) {
    let vertical_layout = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]);
    let [row1, row2, row3, row4] = vertical_layout.areas(frame.area());

    draw_row(frame, row1, 0, &model.grid);
    draw_row(frame, row2, 1, &model.grid);
    draw_row(frame, row3, 2, &model.grid);
    draw_row(frame, row4, 3, &model.grid);

    //frame.render_widget();
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
    use super::display_tile_value;

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
}

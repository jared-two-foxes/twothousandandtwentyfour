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
        frame.render_widget(Paragraph::new(grid.get(i, j).unwrap().to_string()), *area);
    }
}

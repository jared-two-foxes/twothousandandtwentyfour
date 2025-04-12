use TwentyFortyEight::Grid, Model;
use ratatui::Frame;


pub fn view(model: &Model, frame: &mut Frame) {
  let vertical_layout = Layout::vertical([
    Constraint::Fill(1),
    Constraint::Fill(1),
    Constraint::Fill(1),
    Constraint::Fill(1)]);
  let [row1,row2,row3,row4] = vertical_layout.area(frame.area());
  
  draw_row(frame, row1, 0, model.grid);
  draw_row(frame, row2, 1, model.grid);
  draw_row(frame, row3, 2, model.grid);
  draw_row(frame, row4, 3, model.grid);
   
  frame.render_widget();
}

fn draw_row(frame: &mut Frame, area: Area, i: usize, grid: Grid<u16>) {
  let layout = Layout::Horizontal([
    Constraint::Fill(1),
    Constraint::Fill(1),
    Constraint::Fill(1),
    Constraint::Fill(1)]);
  let areas = layout.area(area);
  for j in 0..4 {
    frame.render_widget(
      Paragraph::new(
        grid[Vec2::new(i,j)].to_string()),
      areas[j] );
  } 
}
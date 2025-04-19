use TwentyFortyEight::{Model, Message as ModelMessage};

#[derive(Default)]
struct App {
  model: Model,
  should_quit: bool,
}

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let mut app = App::default();
    
    loop {
        terminal.draw(|f| view(&mut model, f))?;
        
        let current_msg = handle_event(&app)?;
        
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        } 
    }

    ratatui::restore();
    app_result
}

struct Message {
  ModelMessage(ModelMessage),
  Quit,
}

fn handle_event(model: &App) -> anyhow::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Left => Some(ModelMessage::Compress(Vec2::new(-1,0))),
        KeyCode::Right => Some(ModelMessage::Compress(Vec2::new(1,0))),
        KeyCode::Up => Some(ModelMessage::Compress(Vec2::new(0,1))),
        KeyCode::Down => Some(ModelMessage::Compress(Vec2::new(0,-1))),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

fn update(app: &mut App, message: Message) -> Option<Message> {
  None
}
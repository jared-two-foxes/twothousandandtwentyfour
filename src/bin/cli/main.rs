use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use twentyfourtyeight::{actions::Direction, Message as ModelMessage, Model};

mod view;

#[derive(Default)]
struct App {
    model: Model,
    high_score: u32,
    should_quit: bool,
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();

    loop {
        if app.should_quit {
            break;
        }

        terminal.draw(|f| view::view(&mut app.model, f))?;

        let mut current_msg = handle_event(&app)?;

        while current_msg.is_some() {
            current_msg = update(&mut app, current_msg.unwrap());
        }
    }

    ratatui::restore();
    Ok(())
}

enum Message {
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
        KeyCode::Left => Some(Message::ModelMessage(ModelMessage::Compress(
            Direction::Left,
        ))),
        KeyCode::Right => Some(Message::ModelMessage(ModelMessage::Compress(
            Direction::Right,
        ))),
        KeyCode::Up => Some(Message::ModelMessage(ModelMessage::Compress(Direction::Up))),
        KeyCode::Down => Some(Message::ModelMessage(ModelMessage::Compress(
            Direction::Down,
        ))),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

fn update(app: &mut App, message: Message) -> Option<Message> {
    match message {
        Message::Quit => {
            app.should_quit = true;
            None
        }
        Message::ModelMessage(model_message) => {
            twentyfourtyeight::actions::update(&mut app.model, model_message)
                .map(|m| Message::ModelMessage(m))
        }
    }
}

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use twentyfourtyeight::{actions::Direction, Message as ModelMessage, Model};

mod view;

struct App {
    model: Model,
    high_score: u32,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            model: Model::new(),
            high_score: 0,
            should_quit: false,
        }
    }

    // One thing I like about free functions is that it makes it clear at the call site if
    // something is being consumed, borrowed, or mutably borrowed which is not always clear
    // otherwise
    fn update(&mut self, message: Message) -> Option<Message> {
        match message {
            Message::Quit => {
                self.should_quit = true;
                None
            }
            Message::ModelMessage(model_message) => {
                twentyfourtyeight::actions::update(&mut self.model, model_message)
                    .map(|m| Message::ModelMessage(m))
            }
        }
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        if app.should_quit {
            break;
        }

        terminal.draw(|f| view::view(&mut app.model, f))?;

        let mut current_msg = handle_event(&app)?;

        while current_msg.is_some() {
            current_msg = app.update(current_msg.unwrap());
        }
    }

    ratatui::restore();
    Ok(())
}

// And rename this like ApplicaitonMessage or something?
enum Message {
    ModelMessage(ModelMessage), //< Maybe rename this game message.
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

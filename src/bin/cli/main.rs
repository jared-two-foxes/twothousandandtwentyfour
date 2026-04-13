use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use twentyfourtyeight::{actions::Direction, model::State, Message as ModelMessage, Model};

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

        let mut current_msg = filter_message_for_state(app.model.state, handle_event(&app)?);

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

fn handle_event(app: &App) -> anyhow::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(app.model.state, key));
            }
        }
    }
    Ok(None)
}

fn handle_key(state: State, key: event::KeyEvent) -> Option<Message> {
    if matches!(state, State::Won | State::Lost) {
        return match key.code {
            KeyCode::Char('q') => Some(Message::Quit),
            _ => None,
        };
    }

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

fn filter_message_for_state(state: State, message: Option<Message>) -> Option<Message> {
    match (state, message) {
        (State::Won | State::Lost, Some(Message::ModelMessage(_))) => None,
        (_, msg) => msg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_model_input_when_won() {
        let msg = Some(Message::ModelMessage(ModelMessage::Compress(Direction::Left)));
        let filtered = filter_message_for_state(State::Won, msg);

        assert!(filtered.is_none());
    }

    #[test]
    fn allows_model_input_when_won_continue() {
        let msg = Some(Message::ModelMessage(ModelMessage::Compress(Direction::Left)));
        let filtered = filter_message_for_state(State::WonContinue, msg);

        assert!(matches!(filtered, Some(Message::ModelMessage(_))));
    }

    #[test]
    fn allows_quit_when_lost() {
        let filtered = filter_message_for_state(State::Lost, Some(Message::Quit));

        assert!(matches!(filtered, Some(Message::Quit)));
    }
}

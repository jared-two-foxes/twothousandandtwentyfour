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
            Message::Continue => {
                self.model.state = State::WonContinue;
                None
            }
            Message::Restart => {
                self.model = Model::new();
                self.should_quit = false;
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
    Continue,
    Restart,
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
    if matches!(state, State::Won) {
        return match key.code {
            KeyCode::Char('c') => Some(Message::Continue),
            KeyCode::Char('r') => Some(Message::Restart),
            KeyCode::Char('q') => Some(Message::Quit),
            _ => None,
        };
    }

    if matches!(state, State::Lost) {
        return match key.code {
            KeyCode::Char('r') => Some(Message::Restart),
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

    #[test]
    fn won_state_accepts_continue_and_restart() {
        let continue_msg = handle_key(
            State::Won,
            event::KeyEvent::new(KeyCode::Char('c'), event::KeyModifiers::NONE),
        );
        let restart_msg = handle_key(
            State::Won,
            event::KeyEvent::new(KeyCode::Char('r'), event::KeyModifiers::NONE),
        );

        assert!(matches!(continue_msg, Some(Message::Continue)));
        assert!(matches!(restart_msg, Some(Message::Restart)));
    }

    #[test]
    fn lost_state_blocks_continue() {
        let msg = handle_key(
            State::Lost,
            event::KeyEvent::new(KeyCode::Char('c'), event::KeyModifiers::NONE),
        );

        assert!(msg.is_none());
    }

    #[test]
    fn restart_resets_model_and_quit_flag() {
        let mut app = App::new();
        app.should_quit = true;
        app.model.state = State::Lost;
        app.model.score = 128;

        let _ = app.update(Message::Restart);

        assert!(matches!(app.model.state, State::Running));
        assert_eq!(app.model.score, 0);
        assert!(!app.should_quit);
    }
}

use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
    Frame,
};
pub mod app;
pub mod tui;
pub use app::*;
use std::fs;
use std::fs::File;
use std::io::prelude;
pub use tui::*;

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let file_path = "todos.json";
    let mut model: Model;
    let mut file: File;
    let mut data: TodoList;
    match fs::metadata(file_path) {
        Ok(_) => {
            {
                file = File::open(file_path)?;
                data = TodoList {
                    list: serde_json::from_reader(file).expect("error while reading file"),
                };
                model = Model {
                    todo_list: data.list,
                    running_state: RunningState::Running,
                    current_entry: None,
                    entry_text: None,
                    interaction_mode: InteractionMode::Viewing,
                }
                //read todo_list in from file.
            }
        }
        Err(_) => {
            model = Model {
                todo_list: Vec::new(),
                running_state: RunningState::Running,
                current_entry: None,
                entry_text: None,
                interaction_mode: InteractionMode::Viewing,
            };
            file = File::create(file_path)?;
        }
    }

    while model.running_state != RunningState::Done {
        //render current view
        terminal.draw(|f| view(&model, f))?;

        //Handle events and map to a message
        let current_msg = handle_event(&model)?;

        //Process updates as long as they return a non-None message
        if current_msg.is_some() {
            model = update(&mut model, current_msg.unwrap());
        }
    }
    //write todo_list to file.
    let write_list = TodoList {
        list: model.todo_list,
    };
    let write_string = serde_json::to_string(&write_list.list)?;
    fs::write("todos.json", write_string)?;
    tui::restore_terminal()?;
    Ok(())
}

fn handle_event(model: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key, model));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent, model: &Model) -> Option<Message> {
    match model.interaction_mode {
        InteractionMode::Viewing => match key.code {
            KeyCode::Up => Some(Message::PreviousItem),
            KeyCode::Down => Some(Message::NextItem),
            KeyCode::Char('f') => Some(Message::MarkDone),
            KeyCode::Char('d') => Some(Message::DeleteItem),
            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Char('u') => Some(Message::MarkUndone),
            KeyCode::Char('n') => Some(Message::InitAdd),
            KeyCode::Char('k') => Some(Message::PreviousItem),
            KeyCode::Char('j') => Some(Message::NextItem),
            _ => None,
        },
        InteractionMode::Input => match key.code {
            KeyCode::Esc => Some(Message::CancelAdd),
            KeyCode::Char(i) => Some(Message::AddChar(i.to_string())),
            KeyCode::Enter => Some(Message::AddItem(model.entry_text.clone().unwrap())),
            KeyCode::Backspace => Some(Message::RemoveChar),
            _ => None,
        },
    }
}

pub use ratatui::layout::*;
pub use ratatui::style::*;
pub use ratatui::widgets::*;
pub use ratatui::Frame;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct TodoItem {
    pub content: String,
    pub done: bool,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum InteractionMode {
    #[default]
    Viewing,
    Input,
}

#[derive(Debug, Default)]
pub struct Model {
    pub todo_list: Vec<TodoItem>,
    pub running_state: RunningState,
    pub current_entry: Option<usize>,
    pub entry_text: Option<String>,
    pub interaction_mode: InteractionMode,
}

#[derive(PartialEq)]
pub enum Message {
    MarkDone,
    MarkUndone,
    DeleteItem,
    NextItem,
    PreviousItem,
    InitAdd,
    CancelAdd,
    AddChar(String),
    RemoveChar,
    AddItem(String),
    Quit,
}

impl Model {
    pub fn new(
        todolist: Vec<TodoItem>,
        runningstate: RunningState,
        currententry: Option<usize>,
        entrytext: Option<String>,
        interactionmode: InteractionMode,
    ) -> Model {
        Model {
            todo_list: todolist,
            running_state: runningstate,
            current_entry: currententry,
            entry_text: entrytext,
            interaction_mode: interactionmode,
        }
    }
}

pub fn update(model: &Model, msg: Message) -> Model {
    match model.interaction_mode {
        InteractionMode::Viewing => match msg {
            Message::NextItem => {
                if model.current_entry.is_some() {
                    if model.current_entry.unwrap() < model.todo_list.len() - 1 {
                        return Model::new(
                            model.todo_list.clone(),
                            model.running_state,
                            Some(model.current_entry.unwrap() + 1),
                            None,
                            model.interaction_mode,
                        );
                    } else if model.current_entry.unwrap() == model.todo_list.len() - 1 {
                        return Model::new(
                            model.todo_list.clone(),
                            model.running_state,
                            Some(0),
                            None,
                            model.interaction_mode,
                        );
                    }
                } else if model.todo_list.len() > 0 {
                    return Model::new(
                        model.todo_list.clone(),
                        model.running_state,
                        Some(0),
                        None,
                        model.interaction_mode,
                    );
                }
            }
            Message::PreviousItem => {
                if model.current_entry.is_some() {
                    if model.current_entry.unwrap() == 0 {
                        return Model::new(
                            model.todo_list.clone(),
                            model.running_state,
                            Some(model.todo_list.len() - 1),
                            None,
                            model.interaction_mode,
                        );
                    } else {
                        return Model::new(
                            model.todo_list.clone(),
                            model.running_state,
                            Some(model.current_entry.unwrap() - 1),
                            None,
                            model.interaction_mode,
                        );
                    }
                } else if model.todo_list.len() > 0 {
                    return Model::new(
                        model.todo_list.clone(),
                        model.running_state,
                        Some(model.todo_list.len() - 1),
                        None,
                        model.interaction_mode,
                    );
                }
            }
            Message::MarkDone => {
                if model.current_entry.is_some() {
                    if model.current_entry.unwrap() <= model.todo_list.len() - 1 {
                        let mut new_vec = model.todo_list.clone();
                        new_vec[model.current_entry.unwrap()].done = true;
                        return Model::new(
                            new_vec,
                            model.running_state,
                            Some(model.current_entry.unwrap()),
                            None,
                            model.interaction_mode,
                        );
                    }
                } else {
                    return Model::new(
                        model.todo_list.clone(),
                        model.running_state,
                        model.current_entry,
                        None,
                        model.interaction_mode,
                    );
                }
            }
            Message::MarkUndone => {
                if model.current_entry.is_some() {
                    if model.current_entry.unwrap() <= model.todo_list.len() - 1 {
                        let mut new_vec = model.todo_list.clone();
                        new_vec[model.current_entry.unwrap()].done = false;
                        return Model::new(
                            new_vec,
                            model.running_state,
                            Some(model.current_entry.unwrap()),
                            None,
                            model.interaction_mode,
                        );
                    }
                } else {
                    return Model::new(
                        model.todo_list.clone(),
                        model.running_state,
                        model.current_entry,
                        None,
                        model.interaction_mode,
                    );
                }
            }
            Message::DeleteItem => {
                if model.current_entry.is_some() {
                    if model.current_entry.unwrap() < model.todo_list.len() - 1 {
                        let mut new_vec = model.todo_list.clone();
                        new_vec.remove(model.current_entry.unwrap() as usize);
                        return Model::new(
                            new_vec,
                            model.running_state,
                            model.current_entry,
                            None,
                            model.interaction_mode,
                        );
                    } else if model.current_entry.unwrap() == model.todo_list.len() - 1 {
                        let mut new_vec = model.todo_list.clone();
                        new_vec.remove(model.current_entry.unwrap() as usize);
                        return Model::new(
                            new_vec,
                            model.running_state,
                            None,
                            None,
                            model.interaction_mode,
                        );
                    }
                } else {
                    return Model::new(
                        model.todo_list.clone(),
                        model.running_state,
                        model.current_entry,
                        None,
                        model.interaction_mode,
                    );
                }
            }
            Message::Quit => {
                return Model::new(
                    model.todo_list.clone(),
                    RunningState::Done,
                    None,
                    None,
                    InteractionMode::Viewing,
                );
            }
            Message::InitAdd => {
                return Model::new(
                    model.todo_list.clone(),
                    RunningState::Running,
                    model.current_entry,
                    Some("".to_string()),
                    InteractionMode::Input,
                );
            }
            _ => {
                return Model::new(
                    model.todo_list.clone(),
                    model.running_state,
                    model.current_entry,
                    model.entry_text.clone(),
                    model.interaction_mode,
                );
            }
        },
        InteractionMode::Input => match msg {
            Message::CancelAdd => {
                return Model::new(
                    model.todo_list.clone(),
                    RunningState::Running,
                    model.current_entry,
                    None,
                    InteractionMode::Viewing,
                );
            }
            Message::AddChar(i) => {
                return Model::new(
                    model.todo_list.clone(),
                    RunningState::Running,
                    model.current_entry,
                    Some(model.entry_text.clone().unwrap() + &i),
                    InteractionMode::Input,
                );
            }
            Message::RemoveChar => {
                if model.entry_text.is_some() {
                    let mut new_string = model.entry_text.clone().unwrap();
                    new_string.pop();
                    return Model::new(
                        model.todo_list.clone(),
                        RunningState::Running,
                        model.current_entry,
                        Some(new_string),
                        InteractionMode::Input,
                    );
                }
            }
            Message::AddItem(i) => {
                let mut new_vec = model.todo_list.clone();
                new_vec.push(TodoItem {
                    content: i,
                    done: false,
                });
                return Model::new(
                    new_vec,
                    RunningState::Running,
                    model.current_entry.clone(),
                    None,
                    InteractionMode::Viewing,
                );
            }
            _ => {}
        },
    }

    Model::new(
        model.todo_list.clone(),
        model.running_state,
        model.current_entry,
        model.entry_text.clone(),
        model.interaction_mode,
    )
}

pub fn view(model: &Model, f: &mut Frame) {
    let mut state = ListState::default().with_selected(model.current_entry);
    let mut item_vec = Vec::new();
    for list_item in model.todo_list.iter() {
        item_vec.push(list_item.content.clone())
    }
    let list = List::new(item_vec)
        .block(
            Block::bordered().title("Todos").title(
                block::Title::from(
                    "Q: quit, N: new item, D: delete item, F: mark finished, U: mark unfinished",
                )
                .alignment(Alignment::Right),
            ),
        )
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);
    f.render_stateful_widget(
        list,
        Rect::new(0, 0, f.size().width, f.size().height),
        &mut state,
    );
    match model.interaction_mode {
        InteractionMode::Viewing => {}
        InteractionMode::Input => {
            if model.entry_text.is_some() {
                let input_dialog = Paragraph::new(model.entry_text.clone().unwrap())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Add a todo")
                            .border_type(BorderType::Rounded),
                    )
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                f.render_widget(
                    input_dialog,
                    Rect::new(
                        f.size().width / 3,
                        f.size().height / 3,
                        f.size().width / 3,
                        f.size().height / 6,
                    ),
                );
            }
        }
    }
}

use std::time::Duration;

use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    widgets::ListState,
};

use crate::todo::TodoItem;

#[derive(PartialEq)]
pub enum Message {
    SelectPrev,
    SelectNext,
    Delete,
    Create,
    Quit,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default)]
pub struct App {
    pub todos: Vec<TodoItem>,
    pub list_state: ListState,
    pub running_state: RunningState,
}

impl App {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default().with_selected(Some(0)),
            ..Default::default()
        }
    }

    pub fn seed_todo(&mut self) {
        self.todos = vec![
            TodoItem::new(
                String::from("First item"),
                String::from("This is my first item ever"),
            ),
            TodoItem::new(
                String::from("Second item"),
                String::from("This is my second item"),
            ),
            TodoItem::new(
                String::from("Third item"),
                String::from("This is my third item"),
            ),
            TodoItem::new(
                String::from("Fourth item"),
                String::from("This is my fourth item"),
            ),
        ];
    }

    pub fn handle_event() -> Result<Option<Message>> {
        if event::poll(Duration::from_millis(250))?
            && let Event::Key(key) = event::read()?
            && key.kind == event::KeyEventKind::Press
        {
            return Ok(Self::handle_key(key));
        }
        Ok(None)
    }

    fn handle_key(key: event::KeyEvent) -> Option<Message> {
        match key.code {
            KeyCode::Char('j') => Some(Message::SelectNext),
            KeyCode::Char('k') => Some(Message::SelectPrev),
            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Char('D') => Some(Message::Delete),
            _ => None,
        }
    }

    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::SelectPrev => {
                self.list_state.select_previous();
            }
            Message::SelectNext => {
                self.list_state.select_next();
            }
            Message::Delete => {
                if let Some(index) = self.list_state.selected() {
                    self.todos.remove(index);
                }
            }
            Message::Quit => {
                self.running_state = RunningState::Done;
            }
            _ => todo!(),
        };
        None
    }
}

use ratatui::prelude::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, HorizontalAlignment, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app_state: &mut App) {
    let [left, right] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .spacing(1)
        .areas(frame.area());

    let todo_items_list = List::new(app_state.todos.iter().map(|item| item.title.as_str()))
        .highlight_style(Style::new().reversed())
        .highlight_symbol("> ")
        .repeat_highlight_symbol(true)
        .block(
            Block::bordered()
                .title(" My todo list ")
                .title_alignment(HorizontalAlignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .fg(Color::LightBlue),
        );

    let description_content = if let Some(todo) = app_state.selected_todo() {
        todo.description.as_str()
    } else {
        "No item is currently selected."
    };

    let description_text = Paragraph::new(description_content).block(
        Block::bordered()
            .title(" Description ")
            .title_alignment(HorizontalAlignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .fg(Color::LightGreen),
    );

    frame.render_widget(description_text, right);
    frame.render_stateful_widget(todo_items_list, left, &mut app_state.list_state);
}

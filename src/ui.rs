use ratatui::prelude::Stylize;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, HorizontalAlignment, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List},
};

use crate::app::App;

pub fn render_ui(frame: &mut Frame, app_state: &mut App) {
    let [left, right] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .areas(frame.area());

    let list = List::new(app_state.todos.iter().map(|item| item.title.clone()))
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

    frame.render_stateful_widget(list, left, &mut app_state.list_state);
}

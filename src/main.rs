use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Direction, HorizontalAlignment, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph},
};

#[derive(Debug, Default)]
struct AppState {
    todos: Vec<TodoItem>,
    list_state: ListState,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_complete: bool,
    title: String,
    description: String,
}

fn main() -> Result<()> {
    let mut app_state = AppState {
        list_state: ListState::default().with_selected(Some(0)),
        ..Default::default()
    };
    app_state.todos.push(TodoItem {
        is_complete: false,
        title: String::from("Todo item 1"),
        description: String::from("Finish this superb application"),
    });
    app_state.todos.push(TodoItem {
        is_complete: false,
        title: String::from("Todo item 2"),
        description: String::from("Finish this superb application twice"),
    });
    app_state.todos.push(TodoItem {
        is_complete: false,
        title: String::from("Todo item 3"),
        description: String::from("Finish this superb application thrice"),
    });
    app_state.todos.push(TodoItem {
        is_complete: false,
        title: String::from("Todo item 4"),
        description: String::from("Finish this superb application fourth"),
    });
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, &mut app_state);
    ratatui::restore();

    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state))?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                event::KeyCode::Char(char) => match char {
                    'j' => app_state.list_state.select_next(),
                    'k' => app_state.list_state.select_previous(),
                    _ => todo!(),
                },
                _ => todo!(),
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [left, right] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .areas(frame.area());

    let list = List::new(app_state.todos.iter().map(|item| item.title.clone()))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .block(
            Block::bordered()
                .title(" My todo list ")
                .title_alignment(HorizontalAlignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .fg(Color::LightBlue),
        );

    let description = if let Some(index) = app_state.list_state.selected() {
        app_state.todos[index].description.to_string()
    } else {
        String::from("")
    };

    let description_container = Paragraph::new(description);

    frame.render_stateful_widget(list, left, &mut app_state.list_state);
    frame.render_widget(description_container, right);
}

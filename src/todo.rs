#[derive(Debug, Default)]
pub enum TodoState {
    #[default]
    Active,
    Done,
}

#[derive(Debug, Default)]
pub struct TodoItem {
    pub title: String,
    pub description: String,
    state: TodoState,
}

impl TodoItem {
    pub fn new(title: String, description: String) -> Self {
        Self {
            title,
            description,
            state: TodoState::default(),
        }
    }

    fn mark_item_as_done(mut self) {
        self.state = TodoState::Done;
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum State {
    Completed,
    Pending,
    Incomplete,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub state: State,
}

impl Task {
    pub fn mark_completed(&mut self) {
        self.state = State::Completed;
    }
    pub fn mark_pending(&mut self) {
        self.state = State::Pending;
    }
    pub fn mark_incomplete(&mut self) {
        self.state = State::Incomplete;
    }
}

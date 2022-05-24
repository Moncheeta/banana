use crate::buffer::action::Action;

// a vector of Actions
#[derive(Clone)]
pub struct History {
    actions: Vec<Action<String>>,
    pub current_action: Option<Action<String>>,
}

impl History {
    // creates new History list
    pub fn new() -> History {
        History {
            actions: Vec::new(),
            current_action: None,
        }
    }
    // to add an action to History
    pub fn add(&mut self, action: Action<String>) {
        self.actions.push(action);
    }
    // to remove an action at index
    pub fn remove(&mut self, index: usize) {
        if index <= self.actions.len() {
            self.actions.remove(index);
        }
    }
    // returns a reference to the latest action performed
    // it will return None if there are no more actions left in the history
    pub fn last(&self) -> Option<&Action<String>> {
        self.actions.last()
    }
    // like last() but also removes the last element
    pub fn pop_last(&mut self) -> Option<Action<String>> {
        self.actions.pop()
    }
    // returns a copy of all the actions performed
    pub fn actions(self) -> Vec<Action<String>> {
        self.actions
    }
}

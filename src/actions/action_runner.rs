use std::collections::VecDeque;

use crate::actions::action::Action;

pub struct ActionRunner {
    actions: VecDeque<Box<dyn Action>>,
}

impl ActionRunner {
    pub fn new() -> Self {
        ActionRunner {
            actions: VecDeque::new(),
        }
    }

    pub fn run_action(&mut self, mut action: Box<dyn Action>) -> Result<(), String> {
        action.run()?;
        self.actions.push_front(action);
        Ok(())
    }

    pub fn undo_action(&mut self) -> Result<(), String> {
        if self.actions.len() == 0 {
            return Ok(());
        }

        self.actions.pop_front().unwrap().undo()?;

        Ok(())
    }
}

include!("tests/action_runner_test.rs");

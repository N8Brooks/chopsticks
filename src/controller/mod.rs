use crate::state::{Action, ChopsticksState};

pub trait Controller {
    fn get_action(&mut self, gamestate: &ChopsticksState) -> Action;
}

pub mod command_prompt;
pub mod random;

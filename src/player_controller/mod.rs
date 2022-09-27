use crate::chopsticks_state::{Action, ChopsticksState};
pub trait PlayerController {
    fn get_action(&mut self, gamestate: &ChopsticksState) -> Action;
}

pub mod controllers {
    pub mod command_prompt;
    pub mod random;
}

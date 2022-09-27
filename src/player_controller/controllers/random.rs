use crate::player_controller::PlayerController;
use crate::chopsticks_state::{ChopsticksState, Action};
use rand::seq::SliceRandom;

pub struct Random;

impl PlayerController for Random {
    fn get_action(&mut self, gamestate: &ChopsticksState) -> Action {
        let mut actions: Vec<_> = gamestate.actions().collect();
        *actions.choose_mut(&mut rand::thread_rng()).expect("multiple actions")
    }
}

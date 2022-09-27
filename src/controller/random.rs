use crate::controller::Controller;
use crate::state::{Action, ChopsticksState};
use rand::seq::SliceRandom;

pub struct Random;

impl Controller for Random {
    fn get_action(&mut self, gamestate: &ChopsticksState) -> Action {
        let mut actions: Vec<_> = gamestate.actions().collect();
        *actions
            .choose_mut(&mut rand::thread_rng())
            .expect("multiple actions")
    }
}

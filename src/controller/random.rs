use crate::{action, controller::Controller, state::State, state_space::StateSpace};
use rand::seq::SliceRandom;

#[derive(Clone, Default)]
pub struct Random;

impl<const N: usize, T: StateSpace<N>> Controller<N, T> for Random {
    fn get_action(&mut self, gamestate: &State<N, T>) -> action::Action<N, T> {
        let mut actions: Vec<_> = gamestate.actions().collect();
        *actions
            .choose_mut(&mut rand::thread_rng())
            .expect("multiple actions")
    }
}

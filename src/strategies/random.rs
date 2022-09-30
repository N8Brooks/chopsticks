use crate::{state, state_space};
use rand::seq::SliceRandom;

/// Random action of all potential next actions
#[derive(Clone, Default)]
pub struct Random;

impl<const N: usize, T: state_space::StateSpace<N>> super::Strategy<N, T> for Random {
    fn get_action(&mut self, gamestate: &state::State<N, T>) -> state::action::Action<N, T> {
        let mut actions: Vec<_> = gamestate.iter_actions().collect();
        *actions
            .choose_mut(&mut rand::thread_rng())
            .expect("multiple actions")
    }
}

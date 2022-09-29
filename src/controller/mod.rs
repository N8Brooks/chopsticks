use crate::{action, state::State, state_space::StateSpace};

/// 'get_action provider' or an individual player
pub trait Controller<const N: usize, T: StateSpace<N>> {
    fn get_action(&mut self, state: &State<N, T>) -> action::Action<N, T>;
}

pub mod command_prompt;
pub mod random;

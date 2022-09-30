use crate::{state, state_space};

pub mod command_prompt;
pub mod pure_monte_carlo;
pub mod random;

/// 'get_action provider' or an individual player
pub trait Controller<const N: usize, T: state_space::StateSpace<N>> {
    fn get_action(&mut self, state: &state::State<N, T>) -> state::action::Action<N, T>;
}


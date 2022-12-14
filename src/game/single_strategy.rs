pub use crate::game::Game;
use crate::{state, state_space, strategies};

// One controller determines all moves for a game.
pub struct SingleStrategy<'a, const N: usize, T: state_space::StateSpace<N>> {
    pub strategy: &'a mut dyn strategies::Strategy<N, T>,
    pub state: state::State<N, T>,
    pub history: Vec<state::action::Action<N, T>>,
}

impl<'a, const N: usize, T: state_space::StateSpace<N>> SingleStrategy<'a, N, T> {
    pub fn new(
        state: state::State<N, T>,
        strategy: &'a mut dyn strategies::Strategy<N, T>,
    ) -> SingleStrategy<'a, N, T> {
        SingleStrategy {
            strategy,
            state,
            history: Vec::new(),
        }
    }
}

impl<'a, const N: usize, T: state_space::StateSpace<N>> Game<N, T> for SingleStrategy<'a, N, T> {
    fn get_action(&mut self) -> Option<state::action::Action<N, T>> {
        match self.state.get_status() {
            state::status::Status::Turn { i: _ } => Some(self.strategy.get_action(&self.state)),
            _ => None,
        }
    }

    fn play_action(
        &mut self,
        action: &state::action::Action<N, T>,
    ) -> Result<(), state::action::ActionError> {
        self.history.push(*action);
        self.state.play_action(action)
    }

    fn get_state(&self) -> &state::State<N, T> {
        &self.state
    }
}

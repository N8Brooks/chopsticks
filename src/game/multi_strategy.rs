pub use crate::game::Game;
use crate::{strategies, state, state_space};

/// Each player's actions is determined by its own controller.
pub struct MultiStrategy<const N: usize, T: state_space::StateSpace<N>> {
    pub strategies: [Box<dyn strategies::Strategy<N, T>>; N], // could be Rc RefCell for player re-use
    pub state: state::State<N, T>,
    pub history: Vec<state::action::Action<N, T>>,
}

impl<const N: usize, T: state_space::StateSpace<N>> MultiStrategy<N, T> {
    pub fn new(
        state: state::State<N, T>,
        strategies: [Box<dyn strategies::Strategy<N, T>>; N],
    ) -> MultiStrategy<N, T> {
        MultiStrategy {
            strategies,
            state,
            history: Vec::new(),
        }
    }
}

impl<const N: usize, T: state_space::StateSpace<N>> Game<N, T> for MultiStrategy<N, T> {
    fn get_action(&mut self) -> Option<state::action::Action<N, T>> {
        match self.state.get_status() {
            state::status::Status::Turn { id } => Some(self.strategies[id].get_action(&self.state)),
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

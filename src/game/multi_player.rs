pub use crate::game::Game;
use crate::{controller, state, state_space};

/// Each player's actions is determined by its own controller.
pub struct MultiPlayer<const N: usize, T: state_space::StateSpace<N>> {
    pub players: [Box<dyn controller::Controller<N, T>>; N], // could be Rc RefCell for player re-use
    pub state: state::State<N, T>,
    pub history: Vec<state::action::Action<N, T>>,
}

impl<const N: usize, T: state_space::StateSpace<N>> MultiPlayer<N, T> {
    pub fn new(
        state: state::State<N, T>,
        players: [Box<dyn controller::Controller<N, T>>; N],
    ) -> MultiPlayer<N, T> {
        MultiPlayer {
            players,
            state,
            history: Vec::new(),
        }
    }
}

impl<const N: usize, T: state_space::StateSpace<N>> Game<N, T> for MultiPlayer<N, T> {
    fn get_action(&mut self) -> Option<state::action::Action<N, T>> {
        match self.state.get_status() {
            state::status::Status::Turn { id } => Some(self.players[id].get_action(&self.state)),
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

pub use crate::game::Game;
use crate::{controller, state, state_space};

// One controller determines all moves for a game.
pub struct SinglePlayer<'a, const N: usize, T: state_space::StateSpace<N>> {
    pub player: &'a mut dyn controller::Controller<N, T>,
    pub state: state::State<N, T>,
    pub history: Vec<state::action::Action<N, T>>,
}

impl<'a, const N: usize, T: state_space::StateSpace<N>> SinglePlayer<'a, N, T> {
    pub fn new(
        state: state::State<N, T>,
        player: &'a mut dyn controller::Controller<N, T>,
    ) -> SinglePlayer<'a, N, T> {
        SinglePlayer {
            player,
            state,
            history: Vec::new(),
        }
    }
}

impl<'a, const N: usize, T: state_space::StateSpace<N>> Game<N, T> for SinglePlayer<'a, N, T> {
    fn get_action(&mut self) -> Option<state::action::Action<N, T>> {
        match self.state.get_status() {
            state::status::Status::Turn { id: _ } => Some(self.player.get_action(&self.state)),
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

use crate::{state, state_space};

// A trait may be over-engineering the problem at hand.

pub mod multi_player;
pub mod single_player;

/// Encapsulates gameplay within a certain statespace amoung players.
pub trait Game<const N: usize, T: state_space::StateSpace<N>> {
    fn get_action(&mut self) -> Option<state::action::Action<N, T>>;

    fn play_action(
        &mut self,
        action: &state::action::Action<N, T>,
    ) -> Result<(), state::action::ActionError>;
}

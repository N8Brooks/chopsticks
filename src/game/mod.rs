use crate::{state, state_space};

// A trait may be over-engineering the problem at hand.

pub mod multi_strategy;
pub mod single_strategy;

/// Encapsulates gameplay within a certain statespace amoung players.
pub trait Game<const N: usize, T: state_space::StateSpace<N>> {
    fn get_action(&mut self) -> Option<state::action::Action<N, T>>;

    fn play_action(
        &mut self,
        action: &state::action::Action<N, T>,
    ) -> Result<(), state::action::ActionError>;

    fn get_state(&self) -> &state::State<N, T>;

    /// The rank in `1..=N` of each player or `N` if they were already dead
    fn get_rankings(&mut self) -> [usize; N] {
        let mut ranks = [N; N];
        while let state::status::Status::Turn { i: _ } = self.get_state().get_status() {
            if self.get_state().is_loop_state() {
                break;
            }
            let action = self.get_action().expect("ongoing game");
            self.play_action(&action).expect("valid action");
            let player_ids: Vec<_> = self.get_state().iter_player_indexes().collect();
            let n_players = player_ids.len();
            for id in player_ids {
                ranks[id] = n_players;
            }
        }
        ranks
    }
}

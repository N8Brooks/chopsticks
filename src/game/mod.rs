use crate::{state, state_space};
use std::collections::HashSet;

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

    fn get_state(&self) -> &state::State<N, T>;

    /// The rank in `1..=N` of each player or `N` if they were already dead
    fn get_rankings(&mut self) -> [usize; N] {
        let mut ranks = [N; N];
        while let state::status::Status::Turn { id: _ } = self.get_state().get_status() {
            if self.get_state().is_loop_state() {
                break;
            }
            let action = self.get_action().expect("ongoing game");
            self.play_action(&action).expect("valid action");
            let player_ids = self.get_player_ids();
            let n_players = player_ids.len();
            for id in player_ids {
                ranks[id] = n_players;
            }
        }
        ranks
    }

    /// ids of players still alive within the context of the game state
    fn get_player_ids(&mut self) -> HashSet<usize> {
        // TODO: this is quite inefficient
        let mut player_ids = HashSet::new();
        for player in self.get_state().players.iter() {
            player_ids.insert(player.id);
        }
        player_ids
    }
}

pub use crate::game::Game;
use crate::{controller, state, state_space};
use std::collections::HashSet;

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

    /// The ranks of each player or `None` if they were already dead
    pub fn get_rankings(&mut self) -> [Option<usize>; N] {
        let mut ranks = [None; N];
        while let state::status::Status::Turn { id: _ } = self.state.get_status() {
            if self.state.is_loop_state() {
                break;
            }
            let action = self.get_action().expect("ongoing game");
            self.state.play_action(&action).expect("valid action");
            let player_ids = self.get_player_ids();
            let n_players = player_ids.len();
            for id in player_ids {
                ranks[id] = Some(n_players);
            }
        }
        ranks
    }

    /// ids of players still alive within the context of the game state
    fn get_player_ids(&self) -> HashSet<usize> {
        // TODO: this is quite inefficient
        let mut player_ids = HashSet::new();
        for player in self.state.players.iter() {
            player_ids.insert(player.id);
        }
        player_ids
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
}

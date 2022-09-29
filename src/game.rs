use crate::{action, controller, state, state_space};

// A trait may be over-engineering the problem at hand.

/// Encapsulates gameplay within a certain statespace amoung players.
pub trait Game<const N: usize, T: state_space::StateSpace<N>> {
    fn get_action(&mut self) -> Option<action::Action<N, T>>;

    fn play_action(&mut self, action: &action::Action<N, T>) -> Result<(), action::Error>;
}

pub mod single_player {
    use std::collections::HashSet;

    use crate::state::status;

    use super::*;

    // One controller determines all moves for a game.
    pub struct SinglePlayer<'a, const N: usize, T: state_space::StateSpace<N>> {
        pub player: &'a mut dyn controller::Controller<N, T>,
        pub state: state::State<N, T>,
        pub history: Vec<action::Action<N, T>>,
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

        pub fn get_rankings(&mut self) -> [Option<usize>; N] {
            let mut ranks = [None; N];
            let player_ids = self.get_player_ids();
            while let status::Status::Turn { id: _ } = self.state.status() {
                let action = self.get_action().expect("ongoing game");
                self.state.play_action(&action).expect("valid action");
                let abbreviation = self.state.abbreviation();
                if abbreviation == "0102" {
                    // never ending
                    break;
                }
                let new_player_ids = self.get_player_ids();
                for &id in player_ids.difference(&new_player_ids) {
                    ranks[id] = Some(player_ids.len());
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
        fn get_action(&mut self) -> Option<action::Action<N, T>> {
            match self.state.status() {
                state::status::Status::Turn { id: _ } => Some(self.player.get_action(&self.state)),
                _ => None,
            }
        }

        fn play_action(&mut self, action: &action::Action<N, T>) -> Result<(), action::Error> {
            self.history.push(*action);
            self.state.play_action(action)
        }
    }
}

pub mod multi_player {
    use super::*;

    /// Each player's actions is determined by its own controller.
    pub struct MultiPlayer<const N: usize, T: state_space::StateSpace<N>> {
        pub players: [Box<dyn controller::Controller<N, T>>; N], // could be Rc RefCell for player re-use
        pub state: state::State<N, T>,
        pub history: Vec<action::Action<N, T>>,
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
        fn get_action(&mut self) -> Option<action::Action<N, T>> {
            match self.state.status() {
                state::status::Status::Turn { id } => {
                    Some(self.players[id].get_action(&self.state))
                }
                _ => None,
            }
        }

        fn play_action(&mut self, action: &action::Action<N, T>) -> Result<(), action::Error> {
            self.history.push(*action);
            self.state.play_action(action)
        }
    }
}

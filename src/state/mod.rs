use crate::action::{self, attack};
use crate::state_space::StateSpace;
use itertools::Itertools;
use std::collections::VecDeque;
use std::marker::PhantomData;

pub mod player;
pub mod status;

pub const N_HANDS: usize = 2;

/// Game state for [chopsticks](https://en.wikipedia.org/wiki/Chopsticks_(hand_game)#Rules).
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct State<const N: usize, T: StateSpace<N>> {
    pub players: VecDeque<player::Player<N, T>>,
    phantom: PhantomData<T>,
}

impl<const N: usize, T: StateSpace<N>> Default for State<N, T> {
    fn default() -> Self {
        State {
            players: (0..N).map(|id| player::Player::new(id)).collect(),
            phantom: PhantomData {},
        }
    }
}

/// Current state in a game of chopsticks.
impl<const N: usize, T: StateSpace<N>> State<N, T> {
    /// The current *player* uses *hand* `a` to attack *player* in `i` turns at *hand* `b`.
    ///
    /// # Errors
    ///
    /// Returns `PlayerIndexOutOfBounds` if `i` is `0` or `i`
    /// Returns `HandIndexOutOfBounds` if `a`, or `b` are out of bounds.
    /// Returns `HandIsNotAlive` if either the attacking or defending *hand* is dead.
    fn attack(&mut self, i: usize, a: usize, b: usize) -> Result<(), action::attack::Error> {
        if i == 0 || i >= self.players.len() {
            println!("Player index oob");
            Err(attack::Error::PlayerIndexOutOfBounds)
        } else if a >= N_HANDS || b >= N_HANDS {
            println!("hand index oob");

            Err(attack::Error::HandIndexOutOfBounds)
        } else {
            let attacker = self.players[0].hands[a];
            let defending_player = &mut self.players[i];
            let defender = &mut defending_player.hands[b];
            if attacker == 0 || *defender == 0 {
                println!("hand is not alive");

                Err(attack::Error::HandIsNotAlive)
            } else {
                *defender = (*defender + attacker) % T::ROLLOVER;
                defending_player.hands.sort_unstable();
                if defending_player.is_eliminated() {
                    self.players.remove(i);
                }
                self.iterate_turn();
                Ok(())
            }
        }
    }

    /// All possible attack actions from the current `GameState`
    pub fn attack_actions(&self) -> impl Iterator<Item = action::Action<N, T>> + '_ {
        self.players
            .iter()
            .enumerate()
            .skip(1)
            .flat_map(move |(i, defender)| {
                let a_indexes = self.players[0].alive_fingers_indexes();
                let b_indexes = defender.alive_fingers_indexes();
                a_indexes
                    .cartesian_product(b_indexes)
                    .map(move |(a, b)| action::Action::Attack { i, a, b })
            })
    }

    /// The *player* transfers or divides *rollover* among their hands.
    ///
    /// # Errors
    ///
    /// Returns `MoveWithoutChange` if the values of `hands` doesn't change.
    /// Returns `InvalidTotalRollover` when the total number of *rollover* has changed.
    /// Returns `InvalidFingerValue` when any *hand* contains an invalid number of *rollover*.
    fn split(&mut self, mut new_hands: [u32; N_HANDS]) -> Result<(), action::split::Error> {
        new_hands.sort_unstable();
        if self.players[0].hands == new_hands {
            Err(action::split::Error::MoveWithoutChange)
        } else if new_hands.iter().sum::<u32>() != self.players[0].hands.iter().sum() {
            Err(action::split::Error::InvalidTotalRollover)
        } else if new_hands
            .iter()
            .any(|hand| !(1..T::ROLLOVER).contains(hand))
        {
            Err(action::split::Error::InvalidFingerValue)
        } else {
            self.players[0].hands = new_hands;
            self.iterate_turn();
            Ok(())
        }
    }

    /// All possible split actions from the current `GameState`
    pub fn split_actions(&self) -> impl Iterator<Item = action::Action<N, T>> + '_ {
        // TODO: extensible
        let total: u32 = self.players[0].hands.iter().sum();
        let start = (total % T::ROLLOVER + 1).max(1);
        let stop = total / 2;
        (start..=stop)
            .map(move |a| -> [u32; N_HANDS] { [a, total - a] })
            .filter(|&new_hands| self.players[0].hands != new_hands)
            .map(|new_hands| action::Action::Split { hands: new_hands })
    }

    /// Transition state as a player's turn
    pub fn play_action(&mut self, action: &action::Action<N, T>) -> Result<(), action::Error> {
        match action {
            _ if self.players.len() <= 1 => Err(action::Error::GameIsOver),
            action::Action::Attack { i, a, b } => {
                self.attack(*i, *a, *b).map_err(action::Error::AttackError)
            }
            action::Action::Split { hands } => {
                self.split(*hands).map_err(action::Error::SplitError)
            }
            _ => panic!("phantom"),
        }
    }

    pub fn actions(&self) -> impl Iterator<Item = action::Action<N, T>> + '_ {
        self.attack_actions().chain(self.split_actions())
    }

    /// Rotates `self.players` to indicate the next *player's* turn
    fn iterate_turn(&mut self) {
        self.players.rotate_left(1);
    }

    /// The 'abbreviation' representation of the game state.
    pub fn abbreviation(&self) -> String {
        self.players
            .iter()
            .flat_map(|player| player.hands.iter().map(|hand| hand.to_string()))
            .collect()
    }

    /// Current game stage
    pub fn status(&self) -> status::Status {
        if self.players.is_empty() {
            panic!("no non-eliminated players remain");
        }
        let id = self.players[0].id;
        if self.players.len() == 1 {
            status::Status::Over { id }
        } else {
            status::Status::Turn { id }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_space::chopsticks::Chopsticks;

    #[test]
    fn two_players() {
        assert_eq!(
            Chopsticks.get_initial_state(),
            State {
                players: VecDeque::from(vec![
                    player::Player {
                        id: 0,
                        hands: [1; 2],
                        phantom: PhantomData {},
                    },
                    player::Player {
                        id: 1,
                        hands: [1; 2],
                        phantom: PhantomData {},
                    },
                ]),
                phantom: PhantomData {},
            }
        );
    }

    #[test]
    fn attack_invalid_index() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.attack(0, 0, 0).is_err());
        assert!(game_state.attack(1, 2, 0).is_err());
        assert!(game_state.attack(1, 0, 2).is_err());
        assert!(game_state.attack(2, 0, 0).is_err());
    }

    #[test]
    fn attacker_is_zero() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[0].hands[0] = 0;
        assert!(game_state.attack(1, 0, 0).is_err());
    }

    #[test]
    fn defender_is_zero() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[1].hands[0] = 0;
        assert!(game_state.attack(1, 0, 0).is_err());
    }

    #[test]
    fn attack_with_one() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.attack(1, 0, 0).is_ok());
        assert_eq!(game_state.players[0].id, 1);
        assert_eq!(game_state.players[0].hands[1], 2);
    }

    #[test]
    fn attack_with_four() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[0].hands[1] = 4;
        assert!(game_state.attack(1, 1, 1).is_ok());
        assert_eq!(game_state.players[0].id, 1);
        assert_eq!(game_state.players[0].hands[0], 0);
    }
    #[test]
    fn split_with_zero() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.split([0, 2]).is_err());
        assert!(game_state.split([2, 0]).is_err());
    }

    #[test]
    fn split_with_five() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[0].hands = [4; 2];
        assert!(game_state.split([5, 3]).is_err());
        assert!(game_state.split([3, 5]).is_err());
    }

    #[test]
    fn split_invalid_total() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.split([1, 2]).is_err());
    }

    #[test]
    fn split_no_update() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[0].hands[1] = 1;
        assert!(game_state.split([1, 2]).is_err());
        assert!(game_state.split([2, 1]).is_err());
    }

    #[test]
    fn valid_splits() {
        let mut game_state = Chopsticks.get_initial_state();
        for (a, b, c, d) in [
            // Divisions
            (0, 2, 1, 1),
            (0, 3, 1, 2),
            (0, 4, 1, 3),
            (0, 4, 2, 2),
            // Transfers
            (1, 3, 2, 2),
            (2, 2, 1, 3),
            (1, 4, 2, 3),
            (2, 3, 1, 4),
            (2, 4, 3, 3),
            (3, 3, 2, 4),
        ] {
            game_state.players[0].hands[0] = a;
            game_state.players[0].hands[1] = b;
            assert!(game_state.split([c, d]).is_ok());
            assert_eq!(game_state.players[1].hands[0], c);
            assert_eq!(game_state.players[1].hands[1], d);
        }
    }

    #[test]
    fn no_winner_id() {
        let game_state = Chopsticks.get_initial_state();
        assert!(matches!(game_state.status(), status::Status::Turn {id: 0}));
    }

    #[test]
    fn short_game() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.attack(1, 0, 1).is_ok()); // 1211
        assert!(game_state.attack(1, 1, 1).is_ok()); // 1312
        assert!(game_state.attack(1, 1, 1).is_ok()); // 0113
        assert!(game_state.attack(1, 1, 1).is_ok()); // 1401
        assert!(game_state.attack(1, 1, 1).is_ok()); // 0014
        assert!(matches!(game_state.status(), status::Status::Over {id: 0}));
    }
}

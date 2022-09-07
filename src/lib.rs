// A word is *italicized* if it is "business logic" for chopsticks.

use std::collections::VecDeque;

/// Configuration for generating a `ChopsticksState`.
pub struct Chopsticks {
    n_players: usize,
    n_hands: usize,
    fingers: u32,
    initial_fingers: u32,
}

/// Game state for [chopsticks](https://en.wikipedia.org/wiki/Chopsticks_(hand_game)#Rules).
#[derive(Debug, PartialEq, Eq)]
pub struct ChopsticksState {
    players: VecDeque<Player>,
    n_hands: usize,
    fingers: u32,
}

/// The position for an individual *player*.
#[derive(Debug, PartialEq, Eq)]
struct Player {
    /// Uniquely identifies player within a `ChopsticksState`.
    id: usize,

    /// A *player's* *hands* sorted in ascending order.
    pub hands: Vec<u32>,
}

/// `ChopsticksState` builder.
impl Chopsticks {
    /// Creates a `ChopsticksState` builder for additional configuration.
    pub fn default() -> Chopsticks {
        Chopsticks {
            n_players: 2,
            n_hands: 2,
            fingers: 5,
            initial_fingers: 1,
        }
    }

    /// Set the number of *players*. Errors if `0`.
    pub fn with_n_players(&mut self, n_players: usize) -> Result<&mut Chopsticks, ValueError> {
        if n_players == 0 {
            Err(ValueError)
        } else {
            self.n_players = n_players;
            Ok(self)
        }
    }

    /// Set the number of *hands* per *player*. Errors if `0`.
    pub fn with_n_hands(&mut self, n_hands: usize) -> Result<&mut Chopsticks, ValueError> {
        if n_hands == 0 {
            Err(ValueError)
        } else {
            self.n_hands = n_hands;
            Ok(self)
        }
    }

    /// Set the number of *fingers* per *player* *hand*. Errors if `0` or incompatible with `initial_fingers`.
    pub fn with_fingers(&mut self, fingers: u32) -> Result<&mut Chopsticks, ValueError> {
        if fingers == 0 || fingers <= self.initial_fingers {
            Err(ValueError)
        } else {
            self.fingers = fingers;
            Ok(self)
        }
    }

    /// The initial number of *fingers for *hands*. Errors if incompatible with `fingers`.
    pub fn with_initial_fingers(
        &mut self,
        initial_fingers: u32,
    ) -> Result<&mut Chopsticks, ValueError> {
        if initial_fingers >= self.fingers {
            Err(ValueError)
        } else {
            self.initial_fingers = initial_fingers;
            Ok(self)
        }
    }

    /// Initializes a `ChopsticksState` for the given configuration.
    pub fn build(&self) -> ChopsticksState {
        let Chopsticks {
            n_players, // Are these cloned? I hope so.
            n_hands,
            fingers,
            initial_fingers,
        } = *self;
        ChopsticksState {
            players: (0..n_players)
                .map(|id| Player {
                    id,
                    hands: vec![initial_fingers; n_hands],
                })
                .collect(),
            n_hands,
            fingers,
        }
    }
}

/// Current state in a game of chopsticks.
impl ChopsticksState {
    /// The current *player* uses *hand* `a` to attack *player* in `i` turns at *hand* `b`.
    ///
    /// # Errors
    ///
    /// Returns an error if `i` is `0` or `i`, `a`, or `b` are out of bounds.
    /// Returns an error when the attacking or defending *hands* are dead.
    pub fn attack(&mut self, i: usize, a: usize, b: usize) -> Result<(), ValueError> {
        if i == 0 || i >= self.players.len() || a >= self.n_hands || b >= self.n_hands {
            return Err(ValueError);
        }
        let attacker = self.players[0].hands[a];
        let defending_player = &mut self.players[i];
        let defender = &mut defending_player.hands[b];
        if attacker == 0 || *defender == 0 {
            Err(ValueError)
        } else {
            *defender = (*defender + attacker) % self.fingers;
            defending_player.hands.sort_unstable();
            if defending_player.is_eliminated() {
                self.players.remove(i);
            }
            self.iterate_turn();
            Ok(())
        }
    }

    /// The *player* transfers or divides *fingers* among their hands.
    ///
    /// # Errors
    ///
    /// Returns an error when `new_hands` contains no changes.
    /// Returns an error when the total number of *fingers* has changed.
    /// Returns an error when any *hand* contains an invalid number of *fingers*.
    pub fn split(&mut self, mut new_hands: Vec<u32>) -> Result<(), ValueError> {
        new_hands.sort_unstable();
        if self.players[0].hands == new_hands
            || new_hands.iter().sum::<u32>() != self.players[0].hands.iter().sum()
            || new_hands
                .iter()
                .any(|hand| !(1..self.fingers).contains(hand))
        {
            Err(ValueError)
        } else {
            self.players[0].hands = new_hands;
            self.iterate_turn();
            Ok(())
        }
    }

    /// Rotates `self.players` to indicate the next *player's* turn
    ///
    /// # Panics
    ///
    /// An invalid game state where no non-eliminated players remain panics.
    fn iterate_turn(&mut self) {
        let player = self.players.pop_front().unwrap();
        self.players.push_back(player);
    }

    /// The 'abbreviation' representation of the game state.
    pub fn abbreviation(&self) -> String {
        self.players
            .iter()
            .flat_map(|player| player.hands.iter().map(|hand| hand.to_string()))
            .collect()
    }

    /// The *player* `id` for the current turn.
    ///
    /// # Panics
    ///
    /// An invalid game state where no non-eliminated players remain panics.
    pub fn get_turn(&self) -> usize {
        self.players[0].id
    }

    /// The `id` of the *player* if there is exactly 1 non-eliminated *player*
    pub fn winner_id(&self) -> Option<usize> {
        if self.players.len() == 1 {
            Some(self.players[0].id)
        } else {
            None
        }
    }
}

impl Player {
    /// A *player* is eliminated if both of their *hands* are dead.
    fn is_eliminated(&self) -> bool {
        *self.hands.last().expect("no hands") == 0
    }
}

/// Called with invalid arguments.
#[derive(Debug)]
pub struct ValueError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_players() {
        assert_eq!(
            Chopsticks::default().build(),
            ChopsticksState {
                players: VecDeque::from(vec![
                    Player {
                        id: 0,
                        hands: vec![1; 2],
                    },
                    Player {
                        id: 1,
                        hands: vec![1; 2],
                    },
                ]),
                n_hands: 2,
                fingers: 5,
            }
        );
    }

    #[test]
    fn attack_invalid_index() {
        let mut game_state = Chopsticks::default().build();
        assert!(game_state.attack(0, 0, 0).is_err());
        assert!(game_state.attack(1, 2, 0).is_err());
        assert!(game_state.attack(1, 0, 2).is_err());
        assert!(game_state.attack(2, 0, 0).is_err());
    }

    #[test]
    fn attacker_is_zero() {
        let mut game_state = Chopsticks::default().build();
        game_state.players[0].hands[0] = 0;
        assert!(game_state.attack(1, 0, 0).is_err());
    }

    #[test]
    fn defender_is_zero() {
        let mut game_state = Chopsticks::default().build();
        game_state.players[1].hands[0] = 0;
        assert!(game_state.attack(1, 0, 0).is_err());
    }

    #[test]
    fn attack_with_one() {
        let mut game_state = Chopsticks::default().build();
        assert!(game_state.attack(1, 0, 0).is_ok());
        assert_eq!(game_state.players[0].id, 1);
        assert_eq!(game_state.players[0].hands[1], 2);
    }

    #[test]
    fn attack_with_four() {
        let mut game_state = Chopsticks::default().build();
        game_state.players[0].hands[1] = 4;
        assert!(game_state.attack(1, 1, 1).is_ok());
        assert_eq!(game_state.players[0].id, 1);
        assert_eq!(game_state.players[0].hands[0], 0);
    }
    #[test]
    fn split_with_zero() {
        let mut game_state = Chopsticks::default().build();
        assert!(game_state.split(vec![0, 2]).is_err());
        assert!(game_state.split(vec![2, 0]).is_err());
    }

    #[test]
    fn split_with_five() {
        let mut game_state = Chopsticks::default().build();
        game_state.players[0].hands = vec![4; 2];
        assert!(game_state.split(vec![5, 3]).is_err());
        assert!(game_state.split(vec![3, 5]).is_err());
    }

    #[test]
    fn split_invalid_total() {
        let mut game_state = Chopsticks::default().build();
        assert!(game_state.split(vec![1, 2]).is_err());
    }

    #[test]
    fn split_no_update() {
        let mut game_state = Chopsticks::default().build();
        game_state.players[0].hands[1] = 1;
        assert!(game_state.split(vec![1, 2]).is_err());
        assert!(game_state.split(vec![2, 1]).is_err());
    }

    #[test]
    fn valid_splits() {
        let mut game_state = Chopsticks::default().build();
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
            assert!(game_state.split(vec![c, d]).is_ok());
            assert_eq!(game_state.players[1].hands[0], c);
            assert_eq!(game_state.players[1].hands[1], d);
        }
    }

    #[test]
    fn no_winner_id() {
        let game_state = Chopsticks::default().build();
        assert_eq!(game_state.winner_id(), None);
    }

    #[test]
    fn short_game() {
        let mut game_state = Chopsticks::default().build(); // 1111
        assert!(game_state.attack(1, 0, 1).is_ok()); // 1211
        assert!(game_state.attack(1, 1, 1).is_ok()); // 1312
        assert!(game_state.attack(1, 1, 1).is_ok()); // 0113
        assert!(game_state.attack(1, 1, 1).is_ok()); // 1401
        assert!(game_state.attack(1, 1, 1).is_ok()); // 0014
        dbg!(&game_state);
        assert_eq!(game_state.winner_id(), Some(0));
    }
}

// A word is *italicized* if it is "business logic" for chopsticks.

use std::{collections::VecDeque, ops::AddAssign};

/// Game state for [rollover chopsticks](https://en.wikipedia.org/wiki/Chopsticks_(hand_game)#Rules).
#[derive(Debug, PartialEq, Eq)]
pub struct GameState {
    /// Game position describing the hands for each *players*.
    pub players: VecDeque<Player>,
}

/// The position for an individual *player*.
#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    /// Uniquely identifies player intra-game.
    pub id: usize,

    /// A *player's* *hands*
    pub hands: [Hand; 2],
}

/// A *hand* containing `n` *fingers*.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hand(pub u8);

impl GameState {
    /// Initializes a game state for `n` players.
    ///
    /// # Errors
    ///
    /// Errors when attempting to initialize a `GameState` with no *players*.
    pub fn new(n: usize) -> Result<GameState, ValueError> {
        if n == 0 {
            Err(ValueError)
        } else {
            Ok(GameState {
                players: (0..n)
                    .map(|i| Player {
                        id: i,
                        hands: [Hand(1); 2],
                    })
                    .collect(),
            })
        }
    }

    /// The current *player* uses *hand* `a` to attack *player* in `i` turns at *hand* `b`.
    ///
    /// # Errors
    ///
    /// Returns an error if `i` is `0` or `i`, `a`, or `b` are out of bounds.
    /// Returns an error when the attacking or defending *hands* are dead.
    pub fn attack(&mut self, i: usize, a: usize, b: usize) -> Result<(), ValueError> {
        if i == 0 || i >= self.players.len() || a > 1 || b > 1 {
            return Err(ValueError);
        }
        let attacker = self.players[0].hands[a];
        let defender = self.players[i].hands[b];
        if attacker.is_dead() || defender.is_dead() {
            Err(ValueError)
        } else {
            self.players[i].hands[b] += attacker;
            self.players[i].hands.sort_unstable_by_key(|hand| hand.0);
            if self.players[i].is_eliminated() {
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
    /// Returns an error when one of left or right is not between `1` and `4`
    /// Returns an error when there is an incorrect total number of fingers.
    /// Returns an error if left and right remain unchanged or are swapped.
    pub fn split(&mut self, left: u8, right: u8) -> Result<(), ValueError> {
        if self.players[0].try_swap([left, right]) {
            self.iterate_turn();
            Ok(())
        } else {
            Err(ValueError)
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
            .flat_map(|player| player.hands.iter().map(|hand| hand.0.to_string()))
            .collect()
    }

    /// The `id` of the *player* if there is exactly 1 *player*
    pub fn winner_id(&self) -> Option<usize> {
        if self.players.len() == 1 {
            Some(self.players[0].id)
        } else {
            None
        }
    }
}

impl Player {
    /// A *player* is elemiminated if both of their *hands* are dead.
    fn is_eliminated(&self) -> bool {
        self.hands[1].is_dead()
    }

    /// Attempts to update the *hands* and returns `true` if it was successful
    fn try_swap(&mut self, b: [u8; 2]) -> bool {
        // TODO: preferably, we'd only receive the value of the left hand
        if !b.iter().all(|hand| (1..5).contains(hand)) {
            return false;
        }
        let a = self.hands.map(|hand| hand.0);
        if a.iter().sum::<u8>() == b.iter().sum() && !a.contains(&b[0]) {
            self.hands[0].0 = b[0];
            self.hands[1].0 = b[1];
            self.hands.sort_unstable_by_key(|hand| hand.0);
            true
        } else {
            false
        }
    }
}

impl Hand {
    /// A *hand* becomes *dead* when it has `0` mod `5` *fingers*.
    fn is_dead(self) -> bool {
        self.0 == 0
    }
}

impl AddAssign for Hand {
    /// Update for being attacked with `attacker`.
    fn add_assign(&mut self, attacker: Self) {
        *self = Self((self.0 + attacker.0) % 5)
    }
}

/// Called with invalid arguments.
#[derive(Debug)]
pub struct ValueError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_players() {
        assert!(GameState::new(0).is_err());
    }

    #[test]
    fn two_players() {
        assert_eq!(
            GameState::new(2).unwrap(),
            GameState {
                players: VecDeque::from(vec![
                    Player {
                        id: 0,
                        hands: [Hand(1); 2],
                    },
                    Player {
                        id: 1,
                        hands: [Hand(1); 2],
                    },
                ]),
            }
        );
    }

    #[test]
    fn attack_invalid_index() {
        let mut game_state = GameState::new(2).unwrap();
        assert!(game_state.attack(0, 0, 0).is_err());
        assert!(game_state.attack(1, 2, 0).is_err());
        assert!(game_state.attack(1, 0, 2).is_err());
        assert!(game_state.attack(2, 0, 0).is_err());
    }

    #[test]
    fn attacker_is_zero() {
        let mut game_state = GameState::new(2).unwrap();
        game_state.players[0].hands[0].0 = 0;
        assert!(game_state.attack(1, 0, 0).is_err());
    }

    #[test]
    fn defender_is_zero() {
        let mut game_state = GameState::new(2).unwrap();
        game_state.players[1].hands[0].0 = 0;
        assert!(game_state.attack(1, 0, 0).is_err());
    }

    #[test]
    fn attack_with_one() {
        let mut game_state = GameState::new(2).unwrap();
        assert!(game_state.attack(1, 0, 0).is_ok());
        assert_eq!(game_state.players[0].id, 1);
        assert_eq!(game_state.players[0].hands[1].0, 2);
    }

    #[test]
    fn attack_with_four() {
        let mut game_state = GameState::new(2).unwrap();
        game_state.players[0].hands[1].0 = 4;
        assert!(game_state.attack(1, 1, 1).is_ok());
        assert_eq!(game_state.players[0].id, 1);
        assert_eq!(game_state.players[0].hands[0].0, 0);
    }
    #[test]
    fn split_with_zero() {
        let mut game_state = GameState::new(2).unwrap();
        assert!(game_state.split(0, 2).is_err());
        assert!(game_state.split(2, 0).is_err());
    }

    #[test]
    fn split_with_five() {
        let mut game_state = GameState::new(2).unwrap();
        game_state.players[0].hands = [Hand(4); 2];
        assert!(game_state.split(5, 3).is_err());
        assert!(game_state.split(3, 5).is_err());
    }

    #[test]
    fn split_invalid_total() {
        let mut game_state = GameState::new(2).unwrap();
        assert!(game_state.split(1, 2).is_err());
    }

    #[test]
    fn split_no_update() {
        let mut game_state = GameState::new(2).unwrap();
        game_state.players[0].hands[1].0 = 1;
        assert!(game_state.split(1, 2).is_err());
        assert!(game_state.split(2, 1).is_err());
    }

    #[test]
    fn valid_splits() {
        let mut game_state = GameState::new(2).unwrap();
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
            game_state.players[0].hands[0].0 = a;
            game_state.players[0].hands[1].0 = b;
            assert!(game_state.split(c, d).is_ok());
            assert_eq!(game_state.players[1].hands[0].0, c);
            assert_eq!(game_state.players[1].hands[1].0, d);
        }
    }

    #[test]
    fn no_winner_id() {
        let game_state = GameState::new(2).unwrap();
        assert_eq!(game_state.winner_id(), None);
    }

    #[test]
    fn short_game() {
        let mut game_state = GameState::new(2).unwrap(); // 1111
        assert!(game_state.attack(1, 0, 1).is_ok()); // 1211
        assert!(game_state.attack(1, 1, 1).is_ok()); // 1312
        assert!(game_state.attack(1, 1, 1).is_ok()); // 0113
        assert!(game_state.attack(1, 1, 1).is_ok()); // 1401
        assert!(game_state.attack(1, 1, 1).is_ok()); // 0014
        dbg!(&game_state);
        assert_eq!(game_state.winner_id(), Some(0));
    }
}

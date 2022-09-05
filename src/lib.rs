// A word is *italicized* if it is "business logic" for chopsticks.

use std::ops::AddAssign;

// NEED TO RETHINK GAME STATE TO BE LIKE ABBREVIATION. Need to track original index.

/// Game state for [rollover chopsticks](https://en.wikipedia.org/wiki/Chopsticks_(hand_game)#Rules).
#[derive(Debug, PartialEq, Eq)]
pub struct GameState {
    /// The index of the *player* whose *turn* it is
    pub i: usize,

    /// Game position describing the hands for each *players*.
    pub players: Vec<Player>,
}

/// The position for an individual *player*.
#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    /// A *player's* *hands*
    pub hands: [Hand; 2],
}

/// A *hand* containing `n` *fingers*.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hand(u8);

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
                i: 0,
                players: (0..n)
                    .map(|_| Player {
                        hands: [Hand(1); 2],
                    })
                    .collect(),
            })
        }
    }

    /// The *player* at `i` uses *hand* `a` to attack *player* `j` at *hand* `b`.
    ///
    /// # Errors
    ///
    /// Returns an error if `i` is `j` or any of `j`, `a`, or `b` are out of bounds.
    /// Returns an error when the attacking or defending *hand* is dead.
    pub fn attack(&mut self, j: usize, a: usize, b: usize) -> Result<(), ValueError> {
        if self.i == j || j >= self.players.len() || a > 1 || b > 1 {
            return Err(ValueError);
        }
        let attacker = self.players[self.i].hands[a];
        let defender = self.players[j].hands[b];
        if attacker.is_dead() || defender.is_dead() {
            Err(ValueError)
        } else {
            self.players[j].hands[b] += attacker;
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
        if self.players[self.i].try_swap([left, right]) {
            self.iterate_turn();
            Ok(())
        } else {
            Err(ValueError)
        }
    }

    /// Proceeds `i` to the next non-eliminated *players* turn
    ///
    /// # Panics
    ///
    /// An invalid game state where no non-eliminated players remain panics.
    fn iterate_turn(&mut self) {
        self.i = self
            .players
            .iter()
            .enumerate()
            .cycle()
            .skip(self.i + 1)
            .find(|(_, player)| !player.is_eliminated())
            .expect("no non-eliminated players remain")
            .0;
    }

    /// If there is exactly 1 non-eliminated player their index is returned.
    ///
    /// # Panics
    ///
    /// An invalid game state where no non-eliminated players remain panics.
    pub fn winner_position(&self) -> Option<usize> {
        match self
            .players
            .iter()
            .filter(|player| !player.is_eliminated())
            .count()
        {
            0 => panic!("no non-eliminated players remain"),
            1 => Some(self.i),
            _ => None,
        }
    }
}

impl Player {
    /// A *player* is elemiminated if both of their *hands* are dead.
    fn is_eliminated(&self) -> bool {
        self.hands.iter().all(|hand| hand.is_dead())
    }

    /// Attempts to update the *hands* and returns `true` if it was successful
    fn try_swap(&mut self, b: [u8; 2]) -> bool {
        if !b.iter().all(|hand| (1..5).contains(hand)) {
            return false;
        }
        let a = self.hands.map(|hand| hand.0);
        if a.iter().sum::<u8>() == b.iter().sum() && !a.contains(&b[0]) {
            self.hands[0].0 = b[0];
            self.hands[1].0 = b[1];
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
                players: vec![
                    Player {
                        hands: [Hand(1); 2],
                    },
                    Player {
                        hands: [Hand(1); 2],
                    },
                ],
                i: 0,
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
        assert_eq!(game_state.players[1].hands[0].0, 2);
        assert_eq!(game_state.i, 1);
    }

    #[test]
    fn attack_with_four() {
        let mut game_state = GameState::new(2).unwrap();
        game_state.players[0].hands[1].0 = 4;
        assert!(game_state.attack(1, 1, 1).is_ok());
        assert_eq!(game_state.players[1].hands[1].0, 0);
        assert_eq!(game_state.i, 1);
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
        let mut turn = 0;
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
            let i = turn & 1;
            game_state.players[i].hands[0].0 = a;
            game_state.players[i].hands[1].0 = b;
            assert_eq!(i, game_state.i);
            assert!(game_state.split(c, d).is_ok());
            assert_eq!(game_state.players[i].hands[0].0, c);
            assert_eq!(game_state.players[i].hands[1].0, d);
            turn += 1;
        }
    }

    #[test]
    fn none_winner_position() {
        let game_state = GameState::new(2).unwrap();
        assert!(game_state.winner_position().is_none())
    }

    #[test]
    fn winner_position() {
        let mut game_state = GameState::new(2).unwrap(); // 1111
        assert!(game_state.attack(1, 0, 1).is_ok()); // 1112
        assert!(game_state.attack(0, 1, 1).is_ok()); // 1312
        assert!(game_state.attack(1, 1, 1).is_ok()); // 1310
        assert!(game_state.attack(0, 0, 1).is_ok()); // 1410
        assert!(game_state.attack(1, 1, 0).is_ok()); // 1400
        assert_eq!(game_state.winner_position(), Some(0));
    }
}

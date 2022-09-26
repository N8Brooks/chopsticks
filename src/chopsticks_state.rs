use std::collections::VecDeque;

/// Game state for [chopsticks](https://en.wikipedia.org/wiki/Chopsticks_(hand_game)#Rules).
#[derive(Debug, Eq, PartialEq)]
pub struct ChopsticksState {
    pub players: VecDeque<Player>,
    pub n_hands: usize,
    pub rollover: u32,
}

/// The position for an individual *player*.
#[derive(Debug, Eq, PartialEq)]
pub struct Player {
    /// Uniquely identifies player within a `ChopsticksState`.
    pub id: usize,

    /// A *player's* *hands* sorted in ascending order.
    pub hands: Vec<u32>,
}

pub enum AttackError {
    PlayerIndexOutOfBounds,
    HandIndexOutOfBounds,
    HandIsNotAlive,
}

pub enum SplitError {
    MoveWithoutChange,
    InvalidHandLen,
    InvalidTotalRollover,
    InvalidFingerValue,
}

pub enum ActionError {
    AttackError(AttackError),
    SplitError(SplitError),
}

pub enum Action {
    Attack { i: usize, a: usize, b: usize },
    Split { new_hands: Vec<u32> },
}

/// Current state in a game of chopsticks.
impl ChopsticksState {
    /// The current *player* uses *hand* `a` to attack *player* in `i` turns at *hand* `b`.
    ///
    /// # Errors
    ///
    /// Returns `PlayerIndexOutOfBounds` if `i` is `0` or `i`
    /// Returns `HandIndexOutOfBounds` if `a`, or `b` are out of bounds.
    /// Returns `HandIsNotAlive` if either the attacking or defending *hand* is dead.
    pub fn attack(&mut self, i: usize, a: usize, b: usize) -> Result<(), AttackError> {
        if i == 0 || i >= self.players.len() {
            Err(AttackError::PlayerIndexOutOfBounds)
        } else if a >= self.n_hands || b >= self.n_hands {
            Err(AttackError::HandIndexOutOfBounds)
        } else {
            let attacker = self.players[0].hands[a];
            let defending_player = &mut self.players[i];
            let defender = &mut defending_player.hands[b];
            if attacker == 0 || *defender == 0 {
                Err(AttackError::HandIsNotAlive)
            } else {
                *defender = (*defender + attacker) % self.rollover;
                defending_player.hands.sort_unstable();
                if defending_player.is_eliminated() {
                    self.players.remove(i);
                }
                self.iterate_turn();
                Ok(())
            }
        }
    }

    /// The *player* transfers or divides *rollover* among their hands.
    ///
    /// # Errors
    ///
    /// Returns `MoveWithoutChange` if the values of `hands` doesn't change.
    /// Returns `InvalidTotalRollover` when the total number of *rollover* has changed.
    /// Returns `InvalidFingerValue` when any *hand* contains an invalid number of *rollover*.
    pub fn split(&mut self, mut new_hands: Vec<u32>) -> Result<(), SplitError> {
        new_hands.sort_unstable();
        if self.players[0].hands == new_hands {
            Err(SplitError::MoveWithoutChange)
        } else if new_hands.iter().sum::<u32>() != self.players[0].hands.iter().sum() {
            Err(SplitError::InvalidTotalRollover)
        } else if new_hands
            .iter()
            .any(|hand| !(1..self.rollover).contains(hand))
        {
            Err(SplitError::InvalidFingerValue)
        } else {
            self.players[0].hands = new_hands;
            self.iterate_turn();
            Ok(())
        }
    }

    pub fn apply_action(&mut self, action: Action) -> Result<(), ActionError> {
        match action {
            Action::Attack { i, a, b } => self.attack(i, a, b).map_err(ActionError::AttackError),
            Action::Split { new_hands } => self.split(new_hands).map_err(ActionError::SplitError),
        }
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
    /// A *player* is eliminated if all of their *hands* are dead.
    ///
    /// # Panics
    ///
    /// An invalid `Player` state where the *player* has no hands panics.
    fn is_eliminated(&self) -> bool {
        *self.hands.last().expect("no hands") == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chopsticks::Chopsticks;

    #[test]
    fn two_players() {
        assert_eq!(
            Chopsticks::<u32>::default().build(),
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
                rollover: 5,
            }
        );
    }

    #[test]
    fn attack_invalid_index() {
        let mut game_state = Chopsticks::<u32>::default().build();
        assert!(game_state.attack(0, 0, 0).is_err());
        assert!(game_state.attack(1, 2, 0).is_err());
        assert!(game_state.attack(1, 0, 2).is_err());
        assert!(game_state.attack(2, 0, 0).is_err());
    }

    #[test]
    fn attacker_is_zero() {
        let mut game_state = Chopsticks::<u32>::default().build();
        game_state.players[0].hands[0] = 0;
        assert!(game_state.attack(1, 0, 0).is_err());
    }

    #[test]
    fn defender_is_zero() {
        let mut game_state = Chopsticks::<u32>::default().build();
        game_state.players[1].hands[0] = 0;
        assert!(game_state.attack(1, 0, 0).is_err());
    }

    #[test]
    fn attack_with_one() {
        let mut game_state = Chopsticks::<u32>::default().build();
        assert!(game_state.attack(1, 0, 0).is_ok());
        assert_eq!(game_state.players[0].id, 1);
        assert_eq!(game_state.players[0].hands[1], 2);
    }

    #[test]
    fn attack_with_four() {
        let mut game_state = Chopsticks::<u32>::default().build();
        game_state.players[0].hands[1] = 4;
        assert!(game_state.attack(1, 1, 1).is_ok());
        assert_eq!(game_state.players[0].id, 1);
        assert_eq!(game_state.players[0].hands[0], 0);
    }
    #[test]
    fn split_with_zero() {
        let mut game_state = Chopsticks::<u32>::default().build();
        assert!(game_state.split(vec![0, 2]).is_err());
        assert!(game_state.split(vec![2, 0]).is_err());
    }

    #[test]
    fn split_with_five() {
        let mut game_state = Chopsticks::<u32>::default().build();
        game_state.players[0].hands = vec![4; 2];
        assert!(game_state.split(vec![5, 3]).is_err());
        assert!(game_state.split(vec![3, 5]).is_err());
    }

    #[test]
    fn split_invalid_total() {
        let mut game_state = Chopsticks::<u32>::default().build();
        assert!(game_state.split(vec![1, 2]).is_err());
    }

    #[test]
    fn split_no_update() {
        let mut game_state = Chopsticks::<u32>::default().build();
        game_state.players[0].hands[1] = 1;
        assert!(game_state.split(vec![1, 2]).is_err());
        assert!(game_state.split(vec![2, 1]).is_err());
    }

    #[test]
    fn valid_splits() {
        let mut game_state = Chopsticks::<u32>::default().build();
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
        let game_state = Chopsticks::<u32>::default().build();
        assert_eq!(game_state.winner_id(), None);
    }

    #[test]
    fn short_game() {
        let mut game_state = Chopsticks::<u32>::default().build(); // 1111
        assert!(game_state.attack(1, 0, 1).is_ok()); // 1211
        assert!(game_state.attack(1, 1, 1).is_ok()); // 1312
        assert!(game_state.attack(1, 1, 1).is_ok()); // 0113
        assert!(game_state.attack(1, 1, 1).is_ok()); // 1401
        assert!(game_state.attack(1, 1, 1).is_ok()); // 0014
        assert_eq!(game_state.winner_id(), Some(0));
    }
}
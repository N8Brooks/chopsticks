use crate::state_space::StateSpace;
use itertools::Itertools;
use std::collections::VecDeque;
use std::marker::PhantomData;

pub mod action;
pub mod player;
pub mod status;

/// Number of hands per player
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
    /// The current player uses hand `a` to attack player in `i` turns at hand `b`.
    pub fn play_attack(&mut self, i: usize, a: usize, b: usize) -> Result<(), action::AttackError> {
        if i == 0 || i >= self.players.len() {
            Err(action::AttackError::PlayerIndexOutOfBounds)
        } else if a >= N_HANDS || b >= N_HANDS {
            Err(action::AttackError::HandIndexOutOfBounds)
        } else {
            let attacker = self.players[0].hands[a];
            let defending_player = &mut self.players[i];
            let defender = &mut defending_player.hands[b];
            if attacker == 0 || *defender == 0 {
                Err(action::AttackError::HandIsNotAlive)
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
    pub fn iter_attack_actions(&self) -> impl Iterator<Item = action::Action<N, T>> + '_ {
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

    /// The player transfers or divides rollover among their hands.
    pub fn play_split(&mut self, mut new_hands: [u32; N_HANDS]) -> Result<(), action::SplitError> {
        new_hands.sort_unstable();
        if self.players[0].hands == new_hands {
            Err(action::SplitError::MoveWithoutChange)
        } else if new_hands.iter().sum::<u32>() != self.players[0].hands.iter().sum() {
            Err(action::SplitError::InvalidTotalRollover)
        } else if new_hands
            .iter()
            .any(|hand| !(1..T::ROLLOVER).contains(hand))
        {
            Err(action::SplitError::InvalidFingerValue)
        } else {
            self.players[0].hands = new_hands;
            self.iterate_turn();
            Ok(())
        }
    }

    /// All possible split actions from the current `GameState`
    pub fn iter_split_actions(&self) -> impl Iterator<Item = action::Action<N, T>> + '_ {
        let total: u32 = self.players[0].hands.iter().sum();
        let start = (total % T::ROLLOVER + 1).max(1);
        let stop = total / 2;
        (start..=stop)
            .map(move |a| -> [u32; N_HANDS] { [a, total - a] })
            .filter(|&new_hands| self.players[0].hands != new_hands)
            .map(|new_hands| action::Action::Split { hands: new_hands })
    }

    /// Transform `GameState` with a valid `Action` or errors
    pub fn play_action(
        &mut self,
        action: &action::Action<N, T>,
    ) -> Result<(), action::ActionError> {
        match action {
            _ if self.players.len() <= 1 => Err(action::ActionError::GameIsOver),
            action::Action::Attack { i, a, b } => self
                .play_attack(*i, *a, *b)
                .map_err(action::ActionError::AttackError),
            action::Action::Split { hands } => self
                .play_split(*hands)
                .map_err(action::ActionError::SplitError),
            _ => panic!("phantom"),
        }
    }

    /// All potential actions
    pub fn iter_actions(&self) -> impl Iterator<Item = action::Action<N, T>> + '_ {
        self.iter_attack_actions().chain(self.iter_split_actions())
    }

    /// Rotates `self.players` to indicate the next *player's* turn
    fn iterate_turn(&mut self) {
        self.players.rotate_left(1);
    }

    /// The 'abbreviation' representation of the game state.
    pub fn get_abbreviation(&self) -> String {
        self.players
            .iter()
            .flat_map(|player| player.hands.iter().map(|hand| hand.to_string()))
            .collect()
    }

    /// Current game stage
    pub fn get_status(&self) -> status::Status {
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
                players: VecDeque::from(vec![player::Player::new(0), player::Player::new(1),]),
                phantom: PhantomData {},
            }
        );
    }

    #[test]
    fn attack_invalid_index() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.play_attack(0, 0, 0).is_err());
        assert!(game_state.play_attack(1, 2, 0).is_err());
        assert!(game_state.play_attack(1, 0, 2).is_err());
        assert!(game_state.play_attack(2, 0, 0).is_err());
    }

    #[test]
    fn attacker_is_zero() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[0].hands[0] = 0;
        assert!(game_state.play_attack(1, 0, 0).is_err());
    }

    #[test]
    fn defender_is_zero() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[1].hands[0] = 0;
        assert!(game_state.play_attack(1, 0, 0).is_err());
    }

    #[test]
    fn attack_with_one() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.play_attack(1, 0, 0).is_ok());
        assert_eq!(game_state.players[0].id, 1);
        assert_eq!(game_state.players[0].hands[1], 2);
    }

    #[test]
    fn attack_with_four() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[0].hands[1] = 4;
        assert!(game_state.play_attack(1, 1, 1).is_ok());
        assert_eq!(game_state.players[0].id, 1);
        assert_eq!(game_state.players[0].hands[0], 0);
    }
    #[test]
    fn split_with_zero() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.play_split([0, 2]).is_err());
        assert!(game_state.play_split([2, 0]).is_err());
    }

    #[test]
    fn split_with_five() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[0].hands = [4; 2];
        assert!(game_state.play_split([5, 3]).is_err());
        assert!(game_state.play_split([3, 5]).is_err());
    }

    #[test]
    fn split_invalid_total() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.play_split([1, 2]).is_err());
    }

    #[test]
    fn split_no_update() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[0].hands[1] = 1;
        assert!(game_state.play_split([1, 2]).is_err());
        assert!(game_state.play_split([2, 1]).is_err());
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
            assert!(game_state.play_split([c, d]).is_ok());
            assert_eq!(game_state.players[1].hands[0], c);
            assert_eq!(game_state.players[1].hands[1], d);
        }
    }

    #[test]
    fn no_winner_id() {
        let game_state = Chopsticks.get_initial_state();
        assert!(matches!(
            game_state.get_status(),
            status::Status::Turn { id: 0 }
        ));
    }

    #[test]
    fn short_game() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.play_attack(1, 0, 1).is_ok()); // 1211
        assert!(game_state.play_attack(1, 1, 1).is_ok()); // 1312
        assert!(game_state.play_attack(1, 1, 1).is_ok()); // 0113
        assert!(game_state.play_attack(1, 1, 1).is_ok()); // 1401
        assert!(game_state.play_attack(1, 1, 1).is_ok()); // 0014
        assert!(matches!(
            game_state.get_status(),
            status::Status::Over { id: 0 }
        ));
    }
}

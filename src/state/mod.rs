use crate::state_space::StateSpace;
use itertools::Itertools;

pub mod action;
pub mod player;
pub mod status;

/// Number of hands per player
pub const N_HANDS: usize = 2;

/// Game state for [chopsticks](https://en.wikipedia.org/wiki/Chopsticks_(hand_game)#Rules).
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct State<const N: usize, T: StateSpace<N>> {
    /// Current turn
    pub i: usize,

    /// `Player` state
    pub players: [player::Player<N, T>; N],
}

impl<const N: usize, T: StateSpace<N> + std::fmt::Debug> Default for State<N, T> {
    fn default() -> Self {
        State {
            i: 0,
            players: (0..N)
                .map(|_| player::Player::default())
                .collect::<Vec<_>>()
                .try_into()
                .expect("n players"),
        }
    }
}

/// Current state in a game of chopsticks.
impl<const N: usize, T: StateSpace<N>> State<N, T> {
    /// Player `i` uses hand `a` to attack player `j` at hand `b`.
    pub fn play_attack(
        &mut self,
        i: usize,
        j: usize,
        a: usize,
        b: usize,
    ) -> Result<(), action::AttackError> {
        if i >= self.players.len() || j >= self.players.len() {
            Err(action::AttackError::PlayerIndexOutOfBounds)
        } else if a >= N_HANDS || b >= N_HANDS {
            Err(action::AttackError::HandIndexOutOfBounds)
        } else if i == j {
            Err(action::AttackError::PlayerAttackSelf)
        } else {
            let attacker = self.players[i].hands[a];
            let defending_player = &mut self.players[j];
            let defender = &mut defending_player.hands[b];
            if attacker == 0 || *defender == 0 {
                Err(action::AttackError::HandIsNotAlive)
            } else {
                *defender = (*defender + attacker) % T::ROLLOVER;
                self.play_iterate_turn();
                Ok(())
            }
        }
    }

    /// Player `i` uses hand `a` to attack player `j` at hand `b`.
    pub fn undo_attack(
        &mut self,
        i: usize,
        j: usize,
        a: usize,
        b: usize,
    ) -> Result<(), action::AttackError> {
        if i >= self.players.len() || j >= self.players.len() {
            Err(action::AttackError::PlayerIndexOutOfBounds)
        } else if a >= N_HANDS || b >= N_HANDS {
            Err(action::AttackError::HandIndexOutOfBounds)
        } else if i == j {
            Err(action::AttackError::PlayerAttackSelf)
        } else {
            let attacker = self.players[i].hands[a];
            let defending_player = &mut self.players[j];
            let defender = &mut defending_player.hands[b];
            let updated_defender = (*defender + (T::ROLLOVER - attacker)) % T::ROLLOVER;
            if updated_defender == 0 {
                Err(action::AttackError::HandIsNotAlive)
            } else {
                *defender = updated_defender;
                self.undo_iterate_turn();
                Ok(())
            }
        }
    }

    /// All possible attack actions from the current `GameState`
    pub fn iter_attack_actions(&self) -> impl Iterator<Item = action::Action<N, T>> + '_ {
        self.players
            .iter()
            .enumerate()
            .filter(|(j, _)| self.i != *j)
            .flat_map(move |(j, defender)| {
                let a_indexes = self.players[self.i].iter_alive_fingers_indexes();
                let b_indexes = defender.iter_alive_fingers_indexes();
                a_indexes
                    .cartesian_product(b_indexes)
                    .map(move |(a, b)| action::Action::Attack { i: self.i, j, a, b })
            })
    }

    /// The player transfers or divides rollover among their hands.
    pub fn play_split(
        &mut self,
        i: usize,
        hands_0: [u32; N_HANDS],
        hands_1: [u32; N_HANDS],
    ) -> Result<(), action::SplitError> {
        if hands_0 != self.players[i].hands {
            Err(action::SplitError::ImproperContext)
        } else if hands_0.iter().sorted().eq(&hands_1.iter().sorted()) {
            Err(action::SplitError::MoveWithoutChange)
        } else if hands_0.iter().sum::<u32>() != hands_1.iter().sum() {
            Err(action::SplitError::InvalidTotalFingers)
        } else if hands_1.iter().any(|hand| !(1..T::ROLLOVER).contains(hand)) {
            Err(action::SplitError::InvalidFingerValue)
        } else {
            self.players[i].hands = hands_1;
            self.play_iterate_turn();
            Ok(())
        }
    }

    /// The player transfers or divides rollover among their hands.
    pub fn undo_split(
        &mut self,
        i: usize,
        hands_0: [u32; N_HANDS],
        hands_1: [u32; N_HANDS],
    ) -> Result<(), action::SplitError> {
        if hands_1 != self.players[i].hands {
            Err(action::SplitError::ImproperContext)
        } else if hands_0.iter().sorted().eq(&hands_1.iter().sorted()) {
            Err(action::SplitError::MoveWithoutChange)
        } else if hands_0.iter().sum::<u32>() != hands_1.iter().sum() {
            Err(action::SplitError::InvalidTotalFingers)
        } else if hands_0.iter().any(|hand| !(1..T::ROLLOVER).contains(hand)) {
            Err(action::SplitError::InvalidFingerValue)
        } else {
            self.players[i].hands = hands_0;
            self.undo_iterate_turn();
            Ok(())
        }
    }

    /// All possible split actions from the current `GameState`
    pub fn iter_split_actions(&self) -> impl Iterator<Item = action::Action<N, T>> + '_ {
        let total: u32 = self.players[self.i].hands.iter().sum();
        let start = (total % T::ROLLOVER + 1).max(1);
        let stop = total / 2;
        (start..=stop)
            .map(move |a| -> [u32; N_HANDS] { [a, total - a] })
            .filter(|&hands| {
                !self.players[self.i]
                    .hands
                    .iter()
                    .sorted()
                    .eq(&hands.iter().sorted())
            })
            .map(|hands_1| action::Action::Split {
                i: self.i,
                hands_0: self.players[self.i].hands,
                hands_1,
            })
    }

    /// Transform `GameState` with a valid `Action` or errors
    pub fn play_action(
        &mut self,
        action: &action::Action<N, T>,
    ) -> Result<(), action::ActionError> {
        match action {
            _ if self.iter_player_indexes().count() <= 1 => Err(action::ActionError::GameIsOver),
            _ if action.get_i() != self.i => Err(action::ActionError::WrongTurn),
            action::Action::Attack { i, j, a, b } => self
                .play_attack(*i, *j, *a, *b)
                .map_err(action::ActionError::AttackError),
            action::Action::Split {
                i,
                hands_0,
                hands_1,
            } => self
                .play_split(*i, *hands_0, *hands_1)
                .map_err(action::ActionError::SplitError),
            _ => panic!("expect not phantom"),
        }
    }

    pub fn undo_action(
        &mut self,
        action: &action::Action<N, T>,
    ) -> Result<(), action::ActionError> {
        match action {
            action::Action::Attack { i, j, a, b } => self
                .undo_attack(*i, *j, *a, *b)
                .map_err(action::ActionError::AttackError),
            action::Action::Split {
                i,
                hands_0,
                hands_1,
            } => self
                .undo_split(*i, *hands_0, *hands_1)
                .map_err(action::ActionError::SplitError),
            _ => panic!("expect not phantom"),
        }
    }

    /// All potential actions
    pub fn iter_actions(&self) -> impl Iterator<Item = action::Action<N, T>> + '_ {
        self.iter_attack_actions().chain(self.iter_split_actions())
    }

    /// Updates `i` to indicate the next *player's* turn
    fn play_iterate_turn(&mut self) {
        if matches!(self.get_status(), status::Status::Turn { .. }) {
            self.i = self
                .players
                .iter()
                .enumerate()
                .filter(|(_, player)| !player.is_eliminated())
                .cycle()
                .nth(self.i + 1)
                .expect("multiple players")
                .0;
        }
    }

    /// Updates `i` to indicate the previous player's turn
    fn undo_iterate_turn(&mut self) {
        if matches!(self.get_status(), status::Status::Turn { .. }) {
            self.i = self
                .players
                .iter()
                .enumerate()
                .filter(|(_, player)| !player.is_eliminated())
                .rev()
                .cycle()
                .nth(T::N_PLAYERS - self.i)
                .expect("multiple players")
                .0;
        }
    }

    /// The 'abbreviation' representation of the game state.
    pub fn get_abbreviation(&self) -> String {
        self.players
            .iter()
            .flat_map(|player| player.hands.iter().map(|hand| hand.to_string()))
            .collect()
    }

    /// Current game stage panics with no players
    pub fn get_status(&self) -> status::Status {
        let i = self.i;
        match self.iter_player_indexes().count() {
            0 => panic!("no non-eliminated players"),
            1 => status::Status::Over { i },
            _ => status::Status::Turn { i },
        }
    }

    /// Detects loop state for 2 player with rollover 5
    pub fn is_loop_state(&self) -> bool {
        // Could this be done another way?
        if T::N_PLAYERS != 2 || T::INITIAL_FINGERS != 1 || T::ROLLOVER != 5 {
            panic!("not implemented for the `SpaceState`");
        }
        self.players[0].hands.iter().sorted().eq(&[&0, &1])
            && self.players[1].hands.iter().sorted().eq(&[&0, &2])
            || self.players[0].hands.iter().sorted().eq(&[&0, &2])
                && self.players[1].hands.iter().sorted().eq(&[&0, &1])
    }

    /// Iterate non eliminated player indexes
    pub fn iter_player_indexes(&self) -> impl Iterator<Item = usize> + '_ {
        self.players
            .iter()
            .enumerate()
            .filter(|(_, player)| !player.is_eliminated())
            .map(|(i, _)| i)
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
                i: 0,
                players: [player::Player::default(), player::Player::default()],
            }
        );
    }

    #[test]
    fn attack_invalid_index() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.play_attack(0, 0, 0, 0).is_err());
        assert!(game_state.play_attack(0, 1, 2, 0).is_err());
        assert!(game_state.play_attack(0, 1, 0, 2).is_err());
        assert!(game_state.play_attack(0, 2, 0, 0).is_err());
    }

    #[test]
    fn attacker_is_zero() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[0].hands[0] = 0;
        assert!(game_state.play_attack(0, 1, 0, 0).is_err());
    }

    #[test]
    fn defender_is_zero() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[1].hands[0] = 0;
        assert!(game_state.play_attack(0, 1, 0, 0).is_err());
    }

    #[test]
    fn attack_with_one() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.play_attack(0, 1, 0, 0).is_ok());
        assert_eq!(game_state.i, 1);
        assert_eq!(game_state.players[1].hands[0], 2);
    }

    #[test]
    fn attack_with_four() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[0].hands[1] = 4;
        assert!(game_state.play_attack(0, 1, 1, 1).is_ok());
        assert_eq!(game_state.i, 1);
        assert_eq!(game_state.players[1].hands[1], 0);
    }

    #[test]
    fn split_with_zero() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.play_split(0, [1, 1], [0, 2]).is_err());
        assert!(game_state.play_split(0, [1, 1], [2, 0]).is_err());
    }

    #[test]
    fn split_with_five() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[0].hands = [4; 2];
        assert!(game_state.play_split(0, [4, 4], [5, 3]).is_err());
        assert!(game_state.play_split(0, [4, 4], [3, 5]).is_err());
    }

    #[test]
    fn split_invalid_total() {
        let mut game_state = Chopsticks.get_initial_state();
        assert!(game_state.play_split(0, [1, 1], [1, 2]).is_err());
    }

    #[test]
    fn split_no_update() {
        let mut game_state = Chopsticks.get_initial_state();
        game_state.players[0].hands[1] = 2;
        assert!(game_state.play_split(0, [1, 2], [1, 2]).is_err());
        assert!(game_state.play_split(0, [1, 2], [2, 1]).is_err());
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
            assert!(game_state.play_split(0, [a, b], [c, d]).is_ok());
            assert_eq!(game_state.players[0].hands[0], c);
            assert_eq!(game_state.players[0].hands[1], d);
        }
    }

    #[test]
    fn no_winner_id() {
        let game_state = Chopsticks.get_initial_state();
        assert!(matches!(
            game_state.get_status(),
            status::Status::Turn { i: 0 }
        ));
    }

    #[test]
    fn short_game() {
        let mut game_state = Chopsticks.get_initial_state(); // 1111
        assert!(game_state.play_attack(0, 1, 0, 1).is_ok()); // 1112
        assert!(game_state.play_attack(1, 0, 1, 1).is_ok()); // 1312
        assert!(game_state.play_attack(0, 1, 1, 1).is_ok()); // 1310
        assert!(game_state.play_attack(1, 0, 0, 1).is_ok()); // 1410
        assert!(game_state.play_attack(0, 1, 1, 0).is_ok()); // 1400
        assert!(matches!(
            game_state.get_status(),
            status::Status::Over { i: 0 }
        ));
    }
}

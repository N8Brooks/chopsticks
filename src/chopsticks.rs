use crate::chopsticks_state::*;
use num_traits::int::PrimInt;

#[derive(Debug)]
pub struct ValueError;

/// Configuration for generating a `ChopsticksState`.
pub struct Chopsticks<T: PrimInt> {
    n_players: usize,
    n_hands: usize,
    rollover: u32,
    initial_rollover: u32,
    split_offset: T,
    hands_offset: T,
}

impl<T: PrimInt> Default for Chopsticks<T> {
    /// Creates a `ChopsticksState` builder for additional configuration.
    fn default() -> Chopsticks<T> {
        Chopsticks::new(2, 2, 5, 1).expect("default is ok")
    }
}

/// `ChopsticksState` builder.
impl<T: PrimInt> Chopsticks<T> {
    fn new(
        n_players: usize,
        n_hands: usize,
        rollover: u32,
        initial_rollover: u32,
    ) -> Result<Chopsticks<T>, ValueError> {
        // TODO: validate size of T for serials
        let split_offset = {
            let n_players = T::from(n_players).map(Ok).unwrap_or(Err(ValueError))?;
            let n_hands = T::from(n_hands).map(Ok).unwrap_or(Err(ValueError))?;
            n_players * n_hands * n_hands
        };
        let hands_offset = {
            let rollover = T::from(rollover).map(Ok).unwrap_or(Err(ValueError))?;
            rollover.pow(n_hands as u32)
        };
        Ok(Chopsticks {
            n_players,
            n_hands,
            rollover,
            initial_rollover,
            split_offset,
            hands_offset,
        })
    }

    /// Set the number of *players*. Errors if `0`.
    pub fn with_n_players(&mut self, n_players: usize) -> Result<&mut Chopsticks<T>, ValueError> {
        if n_players == 0 {
            Err(ValueError)
        } else {
            self.n_players = n_players;
            Ok(self)
        }
    }

    /// Set the number of *hands* per *player*. Errors if `0`.
    pub fn with_n_hands(&mut self, n_hands: usize) -> Result<&mut Chopsticks<T>, ValueError> {
        if n_hands == 0 {
            Err(ValueError)
        } else {
            self.n_hands = n_hands;
            Ok(self)
        }
    }

    /// Set the number of *rollover* per *player* *hand*. Errors if `0` or incompatible with `initial_rollover`.
    pub fn with_rollover(&mut self, rollover: u32) -> Result<&mut Chopsticks<T>, ValueError> {
        if rollover == 0 || rollover <= self.initial_rollover {
            Err(ValueError)
        } else {
            self.rollover = rollover;
            Ok(self)
        }
    }

    /// The initial number of *rollover for *hands*. Errors if incompatible with `rollover`.
    pub fn with_initial_rollover(
        &mut self,
        initial_rollover: u32,
    ) -> Result<&mut Chopsticks<T>, ValueError> {
        if initial_rollover >= self.rollover {
            Err(ValueError)
        } else {
            self.initial_rollover = initial_rollover;
            Ok(self)
        }
    }

    /// Initializes a `ChopsticksState` for the given configuration.
    pub fn build(&self) -> ChopsticksState {
        let Chopsticks {
            n_players,
            n_hands,
            rollover,
            initial_rollover,
            ..
        } = *self;
        ChopsticksState {
            players: (0..n_players)
                .map(|id| Player {
                    id,
                    hands: vec![initial_rollover; n_hands],
                })
                .collect(),
            n_hands,
            rollover,
        }
    }

    pub fn serialize_action(&self, action: Action) -> T {
        match action {
            Action::Attack { i, a, b } => {
                let n_hands = T::from(self.n_hands).expect("convertable n_hands");
                let mut serial = T::from(i).expect("convertable i");
                serial = serial * n_hands + T::from(a).expect("convertable a");
                serial = serial * n_hands + T::from(b).expect("convertable b");
                serial
            }
            Action::Split { new_hands } => self.serialize_hands(&new_hands) * self.split_offset,
        }
    }

    pub fn serialize_state(&self, state: &ChopsticksState) -> T {
        state.players.iter().fold(T::zero(), |serial, player| {
            serial * self.hands_offset + self.serialize_hands(&player.hands)
        }) * self.split_offset
    }

    fn serialize_hands(&self, hands: &[u32]) -> T {
        let rollover = T::from(self.rollover).expect("convertable rollover");
        hands.iter().fold(T::zero(), |serial, &fingers| {
            let fingers = T::from(fingers).expect("convertable fingers");
            serial * rollover + fingers
        })
    }
}
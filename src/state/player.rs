use super::N_HANDS;
use crate::state_space::StateSpace;
use std::marker::PhantomData;

/// The position for an individual player.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Player<const N: usize, T: StateSpace<N>> {
    /// Uniquely identifies player within a `ChopsticksState`.
    pub id: usize,

    /// A player's hands sorted in ascending order.
    pub hands: [u32; N_HANDS],

    phantom: PhantomData<T>,
}

impl<const N: usize, T: StateSpace<N>> Player<N, T> {
    /// Whether the player has been eliminated
    pub fn is_eliminated(&self) -> bool {
        *self.hands.last().expect("no hands") == 0
    }

    /// Finger indices that are attackable
    pub fn alive_fingers_indexes(&self) -> impl Iterator<Item = usize> + std::clone::Clone + '_ {
        self.hands
            .iter()
            .enumerate()
            .skip_while(|(_, &fingers)| fingers == 0)
            .map(|(i, _)| i)
    }

    /// panics if `id` is not in 0..N for the `StateSpace`
    pub fn new(id: usize) -> Player<N, T> {
        assert!(id < N, "`id` must be less than `N`");
        Player {
            id,
            hands: [T::INITIAL_FINGERS; N_HANDS],
            phantom: PhantomData {},
        }
    }

    /// Numeric 1:1 mapping for valid `Player`
    pub fn serialize(&self) -> u32 {
        self.hands
            .iter()
            .fold(0, |serial, &fingers| serial * T::ROLLOVER + fingers)
    }

    /// Deserialize a `Player` from a *valid* serial
    pub fn from_serial(id: usize, serial: u32) -> Player<N, T> {
        let mut hands = [0; N_HANDS];
        let mut serial = serial;
        let mut i = T::N_HANDS - 1;
        while serial > 0 {
            hands[i] = serial % T::ROLLOVER;
            serial /= T::ROLLOVER;
            i = i.saturating_sub(1);
        }
        Player {
            id,
            hands,
            phantom: PhantomData {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_space::chopsticks::Chopsticks;

    #[test]
    fn serialize() {
        let id = 0;
        let mut serial = 0;
        for hand_0 in 0..Chopsticks::ROLLOVER {
            for hand_1 in 0..Chopsticks::ROLLOVER {
                let hands = [hand_0, hand_1];
                let player = Player::<2, Chopsticks> {
                    id,
                    hands,
                    phantom: PhantomData {},
                };
                assert_eq!(player.serialize(), serial);
                assert_eq!(Player::from_serial(id, serial), player);
                serial += 1;
            }
        }
    }
}

use super::N_HANDS;
use crate::state_space::StateSpace;
use std::marker::PhantomData;

/// The position for an individual player.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Player<const N: usize, T: StateSpace<N>> {
    /// A player's hands sorted in ascending order.
    pub hands: [u32; N_HANDS],

    phantom: PhantomData<T>,
}

impl<const N: usize, T: StateSpace<N>> Player<N, T> {
    /// Whether the player has been eliminated
    pub fn is_eliminated(&self) -> bool {
        self.hands.iter().all(|&hand| hand == 0)
    }

    /// Finger indices that are attackable
    pub fn iter_alive_fingers_indexes(
        &self,
    ) -> impl Iterator<Item = usize> + std::clone::Clone + '_ {
        self.hands
            .iter()
            .enumerate()
            .filter(|(_, &fingers)| fingers != 0)
            .map(|(i, _)| i)
    }
}

impl<const N: usize, T: StateSpace<N>> Default for Player<N, T> {
    fn default() -> Player<N, T> {
        Player {
            hands: [T::INITIAL_FINGERS; N_HANDS],
            phantom: PhantomData {},
        }
    }
}

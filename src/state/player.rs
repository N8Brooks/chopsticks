use super::N_HANDS;
use crate::state_space::StateSpace;
use std::marker::PhantomData;

/// The position for an individual *player*.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Player<const N: usize, T: StateSpace<N>> {
    /// Uniquely identifies player within a `ChopsticksState`.
    pub id: usize,

    /// A *player's* *hands* sorted in ascending order.
    pub hands: [u32; N_HANDS],

    pub phantom: PhantomData<T>,
}

impl<const N: usize, T: StateSpace<N>> Player<N, T> {
    /// A *player* is eliminated if all of their *hands* are dead.
    ///
    /// # Panics
    ///
    /// An invalid `Player` state where the *player* has no hands panics.
    pub fn is_eliminated(&self) -> bool {
        *self.hands.last().expect("no hands") == 0
    }

    pub fn alive_fingers_indexes(&self) -> impl Iterator<Item = usize> + std::clone::Clone + '_ {
        self.hands
            .iter()
            .enumerate()
            .skip_while(|(_, &fingers)| fingers == 0)
            .map(|(i, _)| i)
    }

    pub fn new(id: usize) -> Player<N, T> {
        assert!(id < N, "`id` must be less than `N`");
        Player {
            id,
            hands: [T::INITIAL_FINGERS; N_HANDS],
            phantom: PhantomData {},
        }
    }
}

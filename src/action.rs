use crate::{state::N_HANDS, state_space::StateSpace};
use std::{convert::Infallible, marker::PhantomData};

/// Chopsticks 'move'
#[derive(Copy, Clone, Debug)]
pub enum Action<const N: usize, T: StateSpace<N>> {
    Attack { i: usize, a: usize, b: usize },
    Split { hands: [u32; N_HANDS] },
    _Phantom(Infallible, PhantomData<T>),
}

#[derive(Debug)]
pub enum Error {
    GameIsOver,
    AttackError(attack::Error),
    SplitError(split::Error),
}

pub mod attack {
    #[derive(Debug)]
    pub enum Error {
        PlayerIndexOutOfBounds,
        HandIndexOutOfBounds,
        HandIsNotAlive,
    }
}

pub mod split {
    #[derive(Debug)]
    pub enum Error {
        MoveWithoutChange,
        InvalidHandLen,
        InvalidTotalRollover,
        InvalidFingerValue,
    }
}

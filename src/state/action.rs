use crate::{state, state_space};
use std::{convert::Infallible, marker::PhantomData};

/// Chopsticks 'move'
#[derive(Copy, Clone, Debug)]
pub enum Action<const N: usize, T: state_space::StateSpace<N>> {
    Attack { i: usize, a: usize, b: usize },
    Split { hands: [u32; state::N_HANDS] },
    _Phantom(Infallible, PhantomData<T>),
}

#[derive(Debug)]
pub enum ActionError {
    GameIsOver,
    AttackError(AttackError),
    SplitError(SplitError),
}

#[derive(Debug)]
pub enum AttackError {
    PlayerIndexOutOfBounds,
    HandIndexOutOfBounds,
    HandIsNotAlive,
}

#[derive(Debug)]
pub enum SplitError {
    MoveWithoutChange,
    InvalidHandLen,
    InvalidTotalRollover,
    InvalidFingerValue,
}

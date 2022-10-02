use crate::{state, state_space};
use std::marker::PhantomData;

/// Chopsticks 'move'
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action<const N: usize, T: state_space::StateSpace<N>> {
    Attack {
        i: usize,
        j: usize,
        a: usize,
        b: usize,
    },
    Split {
        i: usize,
        hands_0: [u32; state::N_HANDS],
        hands_1: [u32; state::N_HANDS],
    },
    Phantom(PhantomData<T>),
}

#[derive(Debug)]
pub enum ActionError {
    GameIsOver,
    WrongTurn,
    AttackError(AttackError),
    SplitError(SplitError),
}

#[derive(Debug)]
pub enum AttackError {
    PlayerIndexOutOfBounds,
    HandIndexOutOfBounds,
    HandIsNotAlive,
    PlayerAttackSelf,
}

#[derive(Debug)]
pub enum SplitError {
    ImproperContext,
    MoveWithoutChange,
    InvalidHandLen,
    InvalidTotalFingers,
    InvalidFingerValue,
}

impl<const N: usize, T: state_space::StateSpace<N>> Action<N, T> {
    pub fn get_i(&self) -> usize {
        match self {
            Action::Split { i, .. } => *i,
            Action::Attack { i, .. } => *i,
            Action::Phantom(_) => panic!("expect not phantom"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use state_space::chopsticks::Chopsticks;

    #[test]
    fn get_split_i() {
        let i = 0;
        let action = Action::Split::<2, Chopsticks> {
            i,
            hands_0: [0, 0],
            hands_1: [0, 0],
        };
        assert_eq!(action.get_i(), i);
    }

    #[test]
    fn get_attack_i() {
        let i = 0;
        let action = Action::Attack::<2, Chopsticks> {
            i,
            j: 0,
            a: 0,
            b: 0,
        };
        assert_eq!(action.get_i(), i);
    }
}

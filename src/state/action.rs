use super::N_HANDS;
use crate::{state, state_space};
use std::marker::PhantomData;

/// Chopsticks 'move'
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action<const N: usize, T: state_space::StateSpace<N>> {
    Attack { i: usize, a: usize, b: usize },
    Split { hands: [u32; state::N_HANDS] },
    Phantom(PhantomData<T>),
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
    InvalidTotalFingers,
    InvalidFingerValue,
}

impl<const N: usize, T: state_space::StateSpace<N>> Action<N, T> {
    /// Numeric 1:1 mapping for `Action`. `Action` attributes are not checked for validity.
    pub fn serialize(&self) -> u32 {
        match self {
            Action::Attack { i, a, b } => {
                let mut attack_serial = *i;
                attack_serial = attack_serial * T::N_PLAYERS + a;
                attack_serial = attack_serial * T::N_HANDS + b;
                attack_serial as u32 // offset mapping `attack_serial` to action serial is zero
            }
            Action::Split { hands } => {
                let split_serial = hands
                    .iter()
                    .fold(0, |serial, fingers| serial * T::ROLLOVER + fingers);
                split_serial + T::ATTACK_SERIAL_BASE // offset for `Attack`
            }
            Action::Phantom(_) => panic!("expect not phantom"),
        }
    }

    /// Deserialize an `Action` from a *valid* `serial`
    pub fn from_serial(serial: u32) -> Action<N, T> {
        if serial < T::ATTACK_SERIAL_BASE {
            let mut attack_serial = serial as usize;
            let b = attack_serial % T::N_HANDS;
            attack_serial /= T::N_HANDS;
            let a = attack_serial % T::N_PLAYERS;
            attack_serial /= T::N_PLAYERS;
            let i = attack_serial;
            Action::Attack { i, a, b }
        } else {
            let mut split_serial = serial - T::ATTACK_SERIAL_BASE;
            let mut hands = [0; N_HANDS];
            let mut i = T::N_HANDS - 1;
            while split_serial > 0 {
                hands[i] = split_serial % T::ROLLOVER;
                split_serial /= T::ROLLOVER;
                i = i.saturating_sub(1);
            }
            Action::Split { hands }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_space::StateSpace;
    use state_space::chopsticks::Chopsticks;

    #[test]
    fn attack_serials() {
        let mut serial = 0;
        for i in 0..Chopsticks::N_PLAYERS {
            for a in 0..Chopsticks::N_HANDS {
                for b in 0..Chopsticks::N_HANDS {
                    println!("{serial}");
                    let action = Action::<2, Chopsticks>::Attack { i, a, b };
                    assert_eq!(action.serialize(), serial);
                    assert_eq!(Action::from_serial(serial), action);
                    serial += 1;
                }
            }
        }
    }

    #[test]
    fn split_serials() {
        let mut serial = Chopsticks::ATTACK_SERIAL_BASE;
        for hand_0 in 0..Chopsticks::ROLLOVER {
            for hand_1 in 0..Chopsticks::ROLLOVER {
                println!("{serial}");
                let hands = [hand_0, hand_1];
                let action = Action::<2, Chopsticks>::Split { hands };
                assert_eq!(action.serialize(), serial);
                assert_eq!(Action::from_serial(serial), action);
                serial += 1;
            }
        }
    }

    #[test]
    #[should_panic]
    fn phantom_serial() {
        Action::<2, Chopsticks>::Phantom(PhantomData).serialize();
    }
}

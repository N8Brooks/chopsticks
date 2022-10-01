use crate::state;

/// Number of hands per player. Currently not extensible because of the complexity required to do
/// so.
const N_HANDS: usize = 2;

pub trait StateSpace<const N: usize>: Sized + Copy {
    /// Number of players for a game
    const N_PLAYERS: usize = N;

    /// Number of hands per player
    const N_HANDS: usize = N_HANDS;

    /// A hand is killed when its value is 0 mod `ROLLOVER`
    const ROLLOVER: u32;

    /// Hands are initialized with this number of fingers
    const INITIAL_FINGERS: u32;

    /// The base used for a `Split` `Action` and `Player` state serialization
    const PLAYER_SERIAL_BASE: u32 = Self::ROLLOVER.pow(N_HANDS as u32);

    /// The base used for an `Attack` `Action`. `N_PLAYERS` is 1 higher than what is necessary
    /// because a player cannot attack index 0 which is their own index.
    const ATTACK_SERIAL_BASE: u32 = (Self::N_PLAYERS * N_HANDS * N_HANDS) as u32;

    /// Statically check the base used for an `Action` which may be a `Split` or an `Attack`
    /// against u32
    const ACTION_SERIAL_BASE: u32 = Self::PLAYER_SERIAL_BASE + Self::ATTACK_SERIAL_BASE;

    /// Statically check `State` serial base against u32
    const STATE_SERIAL_BASE: u32 = Self::PLAYER_SERIAL_BASE.pow(Self::N_PLAYERS as u32);

    /// Generate a new chopsticks game instance
    fn get_initial_state(&self) -> state::State<N, Self>
    where
        Self: std::fmt::Debug,
    {
        state::State::default()
    }
}

pub mod chopsticks {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq, Default)]
    pub struct Chopsticks;

    impl StateSpace<2> for Chopsticks {
        const ROLLOVER: u32 = 5;
        const INITIAL_FINGERS: u32 = 1;
    }
}

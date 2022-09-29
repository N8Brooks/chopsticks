use crate::{action, controller, game, state};

pub const N_HANDS: usize = 2;

pub trait StateSpace<const N: usize>: Sized + Copy {
    type Serial;

    const N_PLAYERS: usize = N;
    const ROLLOVER: u32;
    const INITIAL_FINGERS: u32;

    // May need a larger datatype
    const SPLIT_OFFSET: u32 = (N * N_HANDS * N_HANDS) as u32;
    const HANDS_OFFSET: u32 = Self::ROLLOVER.pow(N_HANDS as u32);

    fn serialize_state(&self, state: &state::State<N, Self>) -> u32 {
        state.players.iter().fold(0, |serial, player| {
            serial * Self::HANDS_OFFSET + self.serialize_player(player)
        }) * Self::SPLIT_OFFSET
    }

    fn serialize_action(&self, _action: &action::Action<N, Self>) -> u32 {
        panic!("not implemented");
        // match action {
        //     ChopsticksAction::Attack(_) => S::zero(),
        //     ChopsticksAction::Split(_) => S::zero(),
        // }
    }

    fn serialize_player(&self, player: &state::player::Player<N, Self>) -> u32 {
        player
            .hands
            .iter()
            .fold(0, |serial, &fingers| serial * Self::ROLLOVER + fingers)
    }

    /// Generate a new chopsticks game instance
    fn get_initial_state(&self) -> state::State<N, Self> {
        state::State::default()
    }

    /// Play a game from `state` with actions from `players`
    fn play(
        &self,
        players: [Box<dyn controller::Controller<N, Self>>; N],
    ) -> game::Game<N, Self> {
        game::Game::new(self.get_initial_state(), players)
    }
}

pub mod chopsticks {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq, Default)]
    pub struct Chopsticks;

    impl StateSpace<2> for Chopsticks {
        type Serial = u32;
        const ROLLOVER: u32 = 5;
        const INITIAL_FINGERS: u32 = 1;
    }
}

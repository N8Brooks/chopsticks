use crate::controller;
use crate::game::*;
use crate::state::status::Status;
use crate::{action, state, state_space};
use single_player::SinglePlayer;
use std::marker::PhantomData;

/// Best min-max move according to `n_sims` for each potential move
/// this tends not to work very well because its own future moves are random.
#[derive(Clone)]
pub struct PureMonteCarlo<const N: usize, T: state_space::StateSpace<N>> {
    /// Number of simulations run for each potential move
    n_sims: usize,
    controller: controller::random::Random,
    phantom: PhantomData<T>,
}

impl<const N: usize, T: state_space::StateSpace<N>> controller::Controller<N, T>
    for PureMonteCarlo<N, T>
{
    fn get_action(&mut self, state: &state::State<N, T>) -> action::Action<N, T> {
        let id = match state.status() {
            Status::Turn { id } => id,
            Status::Over { id: _ } => panic!("game is over"),
        };
        state
            .actions()
            .min_by_key(|action| {
                (0..self.n_sims)
                    .map(|_| {
                        let mut state = state.clone();
                        state.play_action(action).expect("valid action");
                        let mut sim_game = SinglePlayer::new(state, &mut self.controller);
                        let ranks = sim_game.get_rankings();
                        ranks[id]
                    })
                    .max()
            })
            .expect("non-zero sims")
    }
}

impl<const N: usize, T: state_space::StateSpace<N>> PureMonteCarlo<N, T> {
    pub fn new(n_sims: usize) -> PureMonteCarlo<N, T> {
        PureMonteCarlo {
            n_sims,
            controller: controller::random::Random {},
            phantom: PhantomData {},
        }
    }
}

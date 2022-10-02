use super::*;
use crate::{game, state, state_space};
use game::Game;
use std::marker::PhantomData;

/// Best min sum of rankings move according to `n_sims` for each potential move
/// this tends not to work very well because its own future moves are random.
#[derive(Clone)]
pub struct PureMonteCarlo<const N: usize, T: state_space::StateSpace<N>> {
    /// Number of simulations run for each potential move
    n_sims: usize,
    strategies: random::Random,
    phantom: PhantomData<T>,
}

impl<const N: usize, T: state_space::StateSpace<N>> Strategy<N, T> for PureMonteCarlo<N, T> {
    fn get_action(&mut self, state: &state::State<N, T>) -> state::action::Action<N, T> {
        let i = match state.get_status() {
            state::status::Status::Turn { i } => i,
            state::status::Status::Over { i: _ } => panic!("game is over"),
        };
        state
            .iter_actions()
            .min_by_key(|action| {
                (0..self.n_sims)
                    .map(|_| {
                        let mut sim_game = game::single_strategy::SingleStrategy::new(
                            state.clone(),
                            &mut self.strategies,
                        );
                        sim_game.play_action(action).expect("valid action");
                        let ranks = sim_game.get_rankings();
                        ranks[i] as u32
                    })
                    .sum::<u32>()
            })
            .expect("non-zero sims")
    }
}

impl<const N: usize, T: state_space::StateSpace<N>> PureMonteCarlo<N, T> {
    pub fn new(n_sims: usize) -> PureMonteCarlo<N, T> {
        PureMonteCarlo {
            n_sims,
            strategies: random::Random {},
            phantom: PhantomData {},
        }
    }
}

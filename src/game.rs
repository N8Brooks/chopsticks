use crate::{action, controller::Controller, state, state_space::StateSpace};

pub struct Game<const N: usize, T: StateSpace<N>> {
    pub players: [Box<dyn Controller<N, T>>; N],
    pub state: state::State<N, T>,
    pub history: Vec<action::Action<N, T>>,
}

impl<const N: usize, T: StateSpace<N>> Game<N, T> {
    pub fn new(state: state::State<N, T>, players: [Box<dyn Controller<N, T>>; N]) -> Game<N, T> {
        Game {
            players,
            state,
            history: Vec::new(),
        }
    }

    pub fn play_next_action(&mut self) -> action::Action<N, T> {
        match self.state.status() {
            state::status::Status::Turn { id } => {
                let action = self.players[id].get_action(&self.state);
                self.state.play_action(&action).expect("valid action");
                self.history.push(action);
                action
            }
            _ => panic!("game is over"),
        }
    }
}

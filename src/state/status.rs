/// Game status
pub enum Status {
    /// The player id of the player for the current turn
    Turn { i: usize },

    /// The winner id after the game is over
    Over { i: usize },
}

impl Status {
    // The current player id regardless of the game status
    pub fn get_i(&self) -> usize {
        match *self {
            Status::Turn { i } => i,
            Status::Over { i } => i,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_turn_i() {
        let i = 0;
        let status = Status::Turn { i };
        assert_eq!(status.get_i(), i);
    }

    #[test]
    fn get_over_i() {
        let i = 0;
        let status = Status::Over { i };
        assert_eq!(status.get_i(), i);
    }
}

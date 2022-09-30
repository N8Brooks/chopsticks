/// Game status
pub enum Status {
    /// The player id of the player for the current turn
    Turn { id: usize },

    /// The winner id after the game is over
    Over { id: usize },
}

impl Status {
    // The current player id regardless of the game status
    pub fn get_id(&self) -> usize {
        match *self {
            Status::Turn { id } => id,
            Status::Over { id } => id,
        }
    }
}

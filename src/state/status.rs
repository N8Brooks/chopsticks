pub enum Status {
    Turn { id: usize },
    Over { id: usize },
}

impl Status {
    pub fn get_id(&self) -> usize {
        match *self {
            Status::Turn { id } => id,
            Status::Over { id } => id,
        }
    }
}

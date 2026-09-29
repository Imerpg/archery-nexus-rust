use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arrow {
    pub score: u8,
    pub order: u8
}

impl Arrow {
    pub fn new(score: u8, order: u8) -> Self {
        Self { score, order }
    }
}
use crate::models::arrow::Arrow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Set {
    pub arrows: Vec<Arrow>,
}

impl Set {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_arrow(&mut self, score: u8) {
        let order = (self.arrows.len() + 1) as u8;
        self.arrows.push(Arrow::new(score, order));
    }

    pub fn total_score(&self) -> u32 {
        self.arrows.iter().map(|a| a.score as u32).sum()
    }
}
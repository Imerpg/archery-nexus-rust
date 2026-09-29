use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetFace {
    Single40cm,
    Trispot40cm,
    Blason60cm,
    Blason80cm,
    Blason122cm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Departure {
    pub id: u8,
    pub name: String,
    pub start_time: String,
    pub distance_meters: u8,
    pub default_target_face: TargetFace,
    pub max_athletes: usize,
}

impl Departure {
    pub fn new(
        id: u8,
        name: &str,
        start_time: &str,
        distance_meters: u8,
        default_target_face: TargetFace,
        max_athletes: usize,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            start_time: start_time.to_string(),
            distance_meters,
            default_target_face,
            max_athletes,
        }
    }
}
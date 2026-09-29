use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistrationStatus {
    Pending,
    Confirmed,
    WaitingList,
    Refused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Athlete {
    pub licence: u64,
    pub first_name: String,
    pub last_name: String,
    pub club: String,
    pub national_average: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    pub athlete: Athlete,
    pub category: String,
    pub status: RegistrationStatus,
}
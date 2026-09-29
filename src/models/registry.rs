use crate::models::athlete::{Athlete, Registration, RegistrationStatus};

#[derive(Debug)]
pub struct CompetitionRegistry {
    pub competition_id: String,
    pub max_capacity: usize,
    pub registrations: Vec<Registration>,
}

impl CompetitionRegistry {
    pub fn new(competition_id: &str, max_capacity: usize) -> Self {
        Self {
            competition_id: competition_id.to_string(),
            max_capacity,
            registrations: Vec::new(),
        }
    }

    pub fn register_athlete(&mut self, athlete: Athlete, category: &str) {
        self.registrations.push(Registration {
            athlete,
            category: category.to_string(),
            status: RegistrationStatus::Pending,
        });
    }

    pub fn apply_cutoff(&mut self) {
        // Tri par moyenne nationale décroissante
        self.registrations.sort_by(|a, b| {
            b.athlete
                .national_average
                .partial_cmp(&a.athlete.national_average)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Attribution des statuts selon la capacité
        for (index, reg) in self.registrations.iter_mut().enumerate() {
            if index < self.max_capacity {
                reg.status = RegistrationStatus::Confirmed;
            } else {
                reg.status = RegistrationStatus::WaitingList;
            }
        }
    }

    pub fn confirmed_athletes(&self) -> Vec<&Registration> {
        self.registrations
            .iter()
            .filter(|r| r.status == RegistrationStatus::Confirmed)
            .collect()
    }

    pub fn waiting_list(&self) -> Vec<&Registration> {
        self.registrations
            .iter()
            .filter(|r| r.status == RegistrationStatus::WaitingList)
            .collect()
    }
}
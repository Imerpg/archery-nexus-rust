use crate::models::athlete::{Athlete, Registration, RegistrationStatus};
use crate::models::departure::Departure;
use crate::models::target::{TargetAssignment, TargetPosition};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CompetitionRegistry {
    pub competition_id: String,
    pub departures: HashMap<u8, Departure>,
    pub registrations: Vec<Registration>,
}

impl CompetitionRegistry {
    pub fn new(competition_id: &str) -> Self {
        Self {
            competition_id: competition_id.to_string(),
            departures: HashMap::new(),
            registrations: Vec::new(),
        }
    }

    pub fn add_departure(&mut self, departure: Departure) {
        self.departures.insert(departure.id, departure);
    }

    pub fn register_athlete(
        &mut self,
        athlete: Athlete,
        category: &str,
        departure_id: u8,
    ) -> Result<(), &'static str> {
        if !self.departures.contains_key(&departure_id) {
            return Err("Le départ spécifié n'existe pas");
        }

        self.registrations.push(Registration {
            athlete,
            category: category.to_string(),
            departure_id,
            status: RegistrationStatus::Pending,
            target_assignment: None,
        });

        Ok(())
    }

    pub fn apply_cutoff(&mut self) {
        let mut by_departure: HashMap<u8, Vec<&mut Registration>> = HashMap::new();

        for reg in self.registrations.iter_mut() {
            by_departure.entry(reg.departure_id).or_default().push(reg);
        }

        for (departure_id, mut regs) in by_departure {
            let max_capacity = self
                .departures
                .get(&departure_id)
                .map(|d| d.max_athletes)
                .unwrap_or(0);

            regs.sort_by(|a, b| {
                b.athlete
                    .national_average
                    .partial_cmp(&a.athlete.national_average)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            for (index, reg) in regs.iter_mut().enumerate() {
                if index < max_capacity {
                    reg.status = RegistrationStatus::Confirmed;
                } else {
                    reg.status = RegistrationStatus::WaitingList;
                    reg.target_assignment = None; // Reset si mis en attente
                }
            }
        }
    }

    pub fn assign_targets(&mut self, departure_id: u8, archers_per_target: u8) {
        let positions = match archers_per_target {
            2 => vec![TargetPosition::A, TargetPosition::B],
            4 => vec![
                TargetPosition::A,
                TargetPosition::B,
                TargetPosition::C,
                TargetPosition::D,
            ],
            _ => vec![TargetPosition::A, TargetPosition::B], // Par défaut 2
        };

        let mut confirmed_regs: Vec<&mut Registration> = self
            .registrations
            .iter_mut()
            .filter(|r| r.departure_id == departure_id && r.status == RegistrationStatus::Confirmed)
            .collect();

        for (index, reg) in confirmed_regs.iter_mut().enumerate() {
            let target_number = ((index as u8) / archers_per_target) + 1;
            let position_idx = (index as usize) % positions.len();
            
            reg.target_assignment = Some(TargetAssignment::new(
                target_number,
                positions[position_idx],
            ));
        }
    }
}
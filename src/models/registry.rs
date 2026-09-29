use crate::models::athlete::{Athlete, Registration, RegistrationStatus};
use crate::models::departure::Departure;
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

    /// Ajoute une vague / départ au concours
    pub fn add_departure(&mut self, departure: Departure) {
        self.departures.insert(departure.id, departure);
    }

    /// Inscrit un archer sur un départ spécifique
    pub fn register_athlete(&mut self, athlete: Athlete, category: &str, departure_id: u8) -> Result<(), &'static str> {
        if !self.departures.contains_key(&departure_id) {
            return Err("Le départ spécifié n'existe pas pour ce concours");
        }

        self.registrations.push(Registration {
            athlete,
            category: category.to_string(),
            departure_id,
            status: RegistrationStatus::Pending,
        });

        Ok(())
    }

    /// Applique le cut-off indépendamment sur chaque départ selon sa capacité propre (`max_athletes`)
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

            // Tri par moyenne nationale décroissante au sein du départ
            regs.sort_by(|a, b| {
                b.athlete
                    .national_average
                    .partial_cmp(&a.athlete.national_average)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            // Application du quota du départ
            for (index, reg) in regs.iter_mut().enumerate() {
                if index < max_capacity {
                    reg.status = RegistrationStatus::Confirmed;
                } else {
                    reg.status = RegistrationStatus::WaitingList;
                }
            }
        }
    }

    /// Récupère la liste des archers confirmés pour un départ donné
    pub fn confirmed_athletes_by_departure(&self, departure_id: u8) -> Vec<&Registration> {
        self.registrations
            .iter()
            .filter(|r| r.status == RegistrationStatus::Confirmed && r.departure_id == departure_id)
            .collect()
    }
}
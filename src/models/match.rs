use crate::models::athlete::Registration;
use crate::models::set::Set;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualificationMatch {
    pub registration: Registration, // Contient l'athlète, son club, sa catégorie et son statut
    pub max_sets: usize,            // Nombre max de volées (ex: 10)
    pub sets: Vec<Set>,             // Liste des volées tirées
}

impl QualificationMatch {
    /// Crée un nouveau match directement à partir d'une inscription d'archer
    pub fn new(registration: Registration, max_sets: usize) -> Self {
        Self {
            registration,
            max_sets,
            sets: Vec::with_capacity(max_sets),
        }
    }

    /// Enregistre une nouvelle volée (Set)
    pub fn add_set(&mut self, set: Set) -> Result<(), &'static str> {
        if self.sets.len() >= self.max_sets {
            return Err("Le nombre maximum de volées pour ce match est déjà atteint");
        }
        self.sets.push(set);
        Ok(())
    }

    /// Calcule le score total cumulé
    pub fn total_score(&self) -> u32 {
        self.sets.iter().map(|s| s.total_score()).sum()
    }

    /// Compte le nombre total de flèches tirées
    pub fn arrow_count(&self) -> usize {
        self.sets.iter().map(|s| s.arrows.len()).sum()
    }

    /// Compte le nombre de 10
    pub fn count_tens(&self) -> u32 {
        self.sets
            .iter()
            .flat_map(|s| &s.arrows)
            .filter(|a| a.score == 10)
            .count() as u32
    }

    /// Indique si le match est terminé
    pub fn is_finished(&self) -> bool {
        self.sets.len() == self.max_sets
    }
}
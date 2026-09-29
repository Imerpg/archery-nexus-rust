use serde::{Deserialize, Serialize};
use std::fmt;

/// Emplacement sur la cible (A, B pour 2 archers/cible ; A, B, C, D pour 4 archers)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetPosition {
    A,
    B,
    C,
    D,
}

/// Poste de tir complet (ex: Cible 3, Position B => "3B")
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetAssignment {
    pub target_number: u8,
    pub position: TargetPosition,
}

impl TargetAssignment {
    pub fn new(target_number: u8, position: TargetPosition) -> Self {
        Self {
            target_number,
            position,
        }
    }
}

impl fmt::Display for TargetAssignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:?}", self.target_number, self.position)
    }
}
use archery_nexus_rust::{
    Athlete, CompetitionRegistry, Departure, RegistrationStatus, TargetFace,
};

fn main() {
    let mut competition = CompetitionRegistry::new("salle_2026");

    // 1. Déclaration des départs
    competition.add_departure(Departure::new(
        1,
        "Départ 1 - Samedi Après-Midi",
        "2026-11-14T14:00:00",
        18,
        TargetFace::Trispot40cm,
        1, // Capacité max : 2 archers
    ));

    competition.add_departure(Departure::new(
        2,
        "Départ 2 - Dimanche Matin",
        "2026-11-15T09:00:00",
        18,
        TargetFace::Single40cm,
        1, // Capacité max : 4 archers
    ));

    // 2. Inscriptions
    let _ = competition.register_athlete(
        Athlete { licence: 101, first_name: "Alice".into(), last_name: "Dupont".into(), club: "Paris".into(), national_average: 570.5 },
        "S1F_CL", 1,
    );
    let _ = competition.register_athlete(
        Athlete { licence: 102, first_name: "Bob".into(), last_name: "Martin".into(), club: "Lyon".into(), national_average: 545.0 },
        "S1H_CL", 1,
    );
    let _ = competition.register_athlete(
        Athlete { licence: 103, first_name: "Charlie".into(), last_name: "Bernard".into(), club: "Marseille".into(), national_average: 588.0 },
        "S1H_CO", 1,
    );
    let _ = competition.register_athlete(
        Athlete { licence: 104, first_name: "David".into(), last_name: "Petit".into(), club: "Toulouse".into(), national_average: 520.0 },
        "S1H_CL", 2,
    );

    // 3. Application du cut-off
    competition.apply_cutoff();

    // 4. Affichage complet des départs
    afficher_departs(&competition);
}

/// Fonction d'affichage détaillée de tous les départs
fn afficher_departs(competition: &CompetitionRegistry) {
    println!("==========================================================");
    println!("        DÉPARTS DU CONCOURS : {}", competition.competition_id);
    println!("==========================================================");

    // Tri des IDs de départ pour un affichage ordonné
    let mut departure_ids: Vec<&u8> = competition.departures.keys().collect();
    departure_ids.sort();

    for id in departure_ids {
        let dep = &competition.departures[id];

        println!("\n▶ DÉPART N°{} : {}", dep.id, dep.name);
        println!("  ├─ Date & Heure : {}", dep.start_time);
        println!("  ├─ Distance     : {} m", dep.distance_meters);
        println!("  ├─ Blason       : {:?}", dep.default_target_face);
        println!("  └─ Capacité     : {} places max", dep.max_athletes);

        println!("  ┌─ ARCHERS :");

        let regs_in_departure: Vec<_> = competition
            .registrations
            .iter()
            .filter(|r| r.departure_id == dep.id)
            .collect();

        if regs_in_departure.is_empty() {
            println!("  │  (Aucun inscrit)");
        } else {
            for reg in regs_in_departure {
                let status_icon = match reg.status {
                    RegistrationStatus::Confirmed => "[CONFIRMÉ]",
                    RegistrationStatus::WaitingList => "[LISTE D'ATTENTE]",
                    RegistrationStatus::Pending => "[EN ATTENTE]",
                    RegistrationStatus::Refused => "[REFUSEé]"
                };

                println!(
                    "  │  {} {} {} (Moy: {}) - Cat: {} - {}",
                    status_icon,
                    reg.athlete.first_name,
                    reg.athlete.last_name,
                    reg.athlete.national_average,
                    reg.category,
                    reg.athlete.club
                );
            }
        }
    }
    println!("\n==========================================================");
}
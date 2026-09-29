use archery_nexus_rust::{
    Athlete, CompetitionRegistry, Departure, RegistrationStatus, TargetFace,
};

fn main() {
    let mut competition = CompetitionRegistry::new("salle_2026");

    // 1. Déclaration du départ avec toutes ses métadonnées
    competition.add_departure(Departure::new(
        1,
        "Départ 1 - Samedi 14h",
        "2026-11-14T14:00:00",
        18,
        TargetFace::Trispot40cm,
        3, // Capacité 4 archers max
    ));

    // 2. Inscriptions des athlètes
    let _ = competition.register_athlete(Athlete { licence: 101, first_name: "Alice".into(), last_name: "Dupont".into(), club: "Paris".into(), national_average: 570.5 }, "S1F_CL", 1);
    let _ = competition.register_athlete(Athlete { licence: 102, first_name: "Bob".into(), last_name: "Martin".into(), club: "Lyon".into(), national_average: 545.0 }, "S1H_CL", 1);
    let _ = competition.register_athlete(Athlete { licence: 103, first_name: "Charlie".into(), last_name: "Bernard".into(), club: "Marseille".into(), national_average: 588.0 }, "S1H_CO", 1);
    let _ = competition.register_athlete(Athlete { licence: 104, first_name: "David".into(), last_name: "Petit".into(), club: "Toulouse".into(), national_average: 520.0 }, "S1H_CL", 1);

    // 3. Application du cut-off selon les capacités
    competition.apply_cutoff();

    // 4. Attribution automatique des postes de tir (2 archers par cible)
    competition.assign_targets(1, 2);

    // 5. Affichage détaillé du Concours et du Départ
    println!("=======================================================================");
    println!(" CONCOURS : {}", competition.competition_id);
    println!("=======================================================================");

    if let Some(dep) = competition.departures.get(&1) {
        println!("▶ DÉPART N°{} : {}", dep.id, dep.name);
        println!("  ├─ Date & Heure : {}", dep.start_time);
        println!("  ├─ Distance     : {} m", dep.distance_meters);
        println!("  ├─ Blason       : {:?}", dep.default_target_face);
        println!("  └─ Capacité     : {} places", dep.max_athletes);
    }

    println!("\n  ┌─ REPARTITION SUR LES CIBLES :");

    for reg in &competition.registrations {
        // Extraction du poste de tir (1A, 1B, 2A, 2B...)
        let position = reg
            .target_assignment
            .as_ref()
            .map(|t| t.to_string())
            .unwrap_or_else(|| "--".to_string());

        let status_str = match reg.status {
            RegistrationStatus::Confirmed => "CONFIRMÉ",
            RegistrationStatus::WaitingList => "EN ATTENTE",
            RegistrationStatus::Pending => "PENDING",
        };

        println!(
            "  │  Emplacement [{:>2}] | [{:<9}] | {} {} ({}) - Cat: {} - Moy: {}",
            position,
            status_str,
            reg.athlete.first_name,
            reg.athlete.last_name,
            reg.athlete.club,
            reg.category,
            reg.athlete.national_average
        );
    }

    println!("=======================================================================");
}
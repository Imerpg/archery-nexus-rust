use archery_nexus_rust::{Athlete, CompetitionRegistry};

fn main() {
    let mut competition = CompetitionRegistry::new("salle_2026", 2);

    competition.register_athlete(
        Athlete {
            licence: 123456,
            first_name: "Alice".into(),
            last_name: "Dupont".into(),
            club: "Paris".into(),
            national_average: 570.5,
        },
        "S1F_CL",
    );

    competition.register_athlete(
        Athlete {
            licence: 654321,
            first_name: "Bob".into(),
            last_name: "Martin".into(),
            club: "Lyon".into(),
            national_average: 545.0,
        },
        "S1H_CL",
    );

    competition.register_athlete(
        Athlete {
            licence: 987654,
            first_name: "Charlie".into(),
            last_name: "Bernard".into(),
            club: "Marseille".into(),
            national_average: 588.0,
        },
        "S1H_CO",
    );

    // Tri et validation du quota
    competition.apply_cutoff();

    println!("=== ARCHERS CONFIRMÉS ===");
    for reg in competition.confirmed_athletes() {
        println!(
            "- {} {} ({}) : Moyenne {}",
            reg.athlete.first_name,
            reg.athlete.last_name,
            reg.category,
            reg.athlete.national_average
        );
    }

    println!("\n=== LISTE D'ATTENTE ===");
    for reg in competition.waiting_list() {
        println!(
            "- {} {} : Moyenne {}",
            reg.athlete.first_name,
            reg.athlete.last_name,
            reg.athlete.national_average
        );
    }
}
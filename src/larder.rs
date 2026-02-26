use crate::data::*;

pub fn container_assignments<'a>(larder: &'a Larder) -> Vec<(&'a str, &'a str)> {
    larder
        .mixes
        .iter()
        .filter_map(|mix| mix.container.as_deref().map(|c| (mix.name.as_str(), c)))
        .collect()
}

pub fn unassigned_containers(larder: &Larder) -> Vec<&Container> {
    let assigned: Vec<&str> = larder
        .mixes
        .iter()
        .filter_map(|m| m.container.as_deref())
        .collect();

    larder
        .containers
        .iter()
        .filter(|c| !assigned.contains(&c.name.as_str()))
        .collect()
}

pub fn print_larder_status(larder: &Larder) {
    println!("=== Larder Status ===\n");

    let assignments = container_assignments(larder);
    println!("Assigned:");
    for (mix, container) in &assignments {
        println!("  {} -> {}", container, mix);
    }

    let free = unassigned_containers(larder);
    println!("\nAvailable ({}):", free.len());
    for c in &free {
        println!("  {} ({}ml {:?})", c.name, c.capacity_ml, c.kind);
    }
    println!();
}

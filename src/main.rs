mod cooking;
mod data;
mod display;
mod larder;
mod processing;
mod seed;
mod sourcing;

fn main() {
    let larder = seed::skog_grautr_larder();

    // Display the Skog Grautr mix
    let mix = larder
        .mixes
        .iter()
        .find(|m| m.name == "Skog Grautr")
        .expect("Skog Grautr mix not found");
    display::print_mix_summary(mix);

    // Processing steps
    let steps = processing::processing_steps_for_mix(&larder, "Skog Grautr");
    processing::print_processing_steps(&steps);

    // Shopping list
    let shopping = sourcing::shopping_list_for_mix(&larder, "Skog Grautr");
    sourcing::print_shopping_list(&shopping);

    // Larder status
    larder::print_larder_status(&larder);

    // Recipe
    if let Some(recipe) = cooking::find_recipe_for_mix(&larder, "Skog Grautr") {
        display::print_recipe(recipe);
    }

    // TOML round-trip
    let toml_output = toml::to_string_pretty(&larder).expect("Failed to serialize larder to TOML");
    println!("=== TOML Output ===\n");
    println!("{}", toml_output);
}

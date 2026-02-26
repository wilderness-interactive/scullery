mod cooking;
mod data;
mod display;
mod larder;
mod processing;
mod seed;
mod sourcing;

fn main() {
    let larder = seed::full_larder();

    // Display each recipe mix and its cooking steps
    for recipe in &larder.recipes {
        let mix = larder.mixes.iter().find(|m| m.name == recipe.mix);
        if let Some(mix) = mix {
            display::print_mix_summary(mix);
        }

        let steps = processing::processing_steps_for_mix(&larder, &recipe.mix);
        if !steps.is_empty() {
            processing::print_processing_steps(&steps);
        }

        display::print_recipe(recipe);
    }

    // Combined order across all recipes
    let order = sourcing::combined_order(&larder);
    sourcing::print_combined_order(&order);

    // Larder status
    larder::print_larder_status(&larder);
}

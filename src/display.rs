use crate::data::*;

pub fn print_mix_summary(mix: &Mix) {
    println!("=== {} ===", mix.name);
    println!("{}\n", mix.description);

    println!("Components:");
    for c in &mix.components {
        let prop = format_proportion(&c.proportion);
        println!("  - {} ({:?}) -- {}", c.ingredient, c.form, prop);
    }

    if let Some(container) = &mix.container {
        println!("\nStored in: {}", container);
    }
    println!();
}

pub fn print_recipe(recipe: &Recipe) {
    println!("--- {} ---", recipe.name);
    println!("Servings: {}\n", recipe.servings);

    for (i, step) in recipe.steps.iter().enumerate() {
        let mut details = Vec::new();
        if let Some(water) = step.water_ml {
            details.push(format!("{}ml water", water));
        }
        if let Some(mins) = step.duration_minutes {
            details.push(format!("{} min", mins));
        }
        if let Some(heat) = &step.heat {
            details.push(format!("{:?}", heat));
        }
        let suffix = if details.is_empty() {
            String::new()
        } else {
            format!(" [{}]", details.join(", "))
        };
        println!("  {}. {}{}", i + 1, step.instruction, suffix);
    }
    println!();
}

fn format_proportion(p: &Proportion) -> String {
    match p {
        Proportion::EqualPart => "equal part".to_owned(),
        Proportion::Grams(g) => format!("{g}g"),
        Proportion::Tablespoons(t) => format!("{t} tbsp"),
        Proportion::Teaspoons(t) => format!("{t} tsp"),
        Proportion::ToTaste => "to taste".to_owned(),
    }
}

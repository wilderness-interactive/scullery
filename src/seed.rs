use crate::data::*;

pub fn skog_grautr_ingredients() -> Vec<Ingredient> {
    vec![
        Ingredient {
            name: "Wheat".to_owned(),
            kind: IngredientKind::Grain,
            form: Form::Whole,
            notes: Some("YQ Wheat — genetically diverse population wheat, grown in Suffolk".to_owned()),
        },
        Ingredient {
            name: "Naked Barley".to_owned(),
            kind: IngredientKind::Grain,
            form: Form::Kibbled,
            notes: Some("Oak Ruby variety, pre-kibbled by Hodmedod's. No processing needed.".to_owned()),
        },
        Ingredient {
            name: "Rye".to_owned(),
            kind: IngredientKind::Grain,
            form: Form::Whole,
            notes: Some("Organic wholegrain rye, grown at Wimpole Hall, Cambridgeshire".to_owned()),
        },
        Ingredient {
            name: "Naked Oats".to_owned(),
            kind: IngredientKind::Grain,
            form: Form::Whole,
            notes: Some("Hulless oats, grown in Scotland. Non-GF (farm also grows cereals). Store cold.".to_owned()),
        },
        Ingredient {
            name: "Porcini Mushrooms".to_owned(),
            kind: IngredientKind::Mushroom,
            form: Form::Dried,
            notes: Some("Whole dried slices — the deep umami earthiness of the forest".to_owned()),
        },
        Ingredient {
            name: "Parsley".to_owned(),
            kind: IngredientKind::Herb,
            form: Form::Dried,
            notes: None,
        },
        Ingredient {
            name: "Onion".to_owned(),
            kind: IngredientKind::Allium,
            form: Form::Whole,
            notes: Some("Home-dehydrated in Ninja Foodi. Shop-bought powder is bitter.".to_owned()),
        },
    ]
}

pub fn skog_grautr_sources() -> Vec<Source> {
    vec![
        Source {
            supplier: "Hodmedod's".to_owned(),
            url: Some("https://hodmedods.co.uk/products/yq-wheat-organic-wholegrain".to_owned()),
            ingredient: "Wheat".to_owned(),
            pack_grams: Some(500.0),
            price_pence: Some(189),
            notes: Some("YQ Wheat, Organic Wholegrain — more diverse kernels visible".to_owned()),
        },
        Source {
            supplier: "Hodmedod's".to_owned(),
            url: Some("https://hodmedods.co.uk/products/kibbled-naked-barley".to_owned()),
            ingredient: "Naked Barley".to_owned(),
            pack_grams: Some(500.0),
            price_pence: Some(199),
            notes: Some("Kibbled Naked Barley — already cracked, ready to use".to_owned()),
        },
        Source {
            supplier: "Hodmedod's".to_owned(),
            url: Some("https://hodmedods.co.uk/products/rye-grain-organic-wholegrain".to_owned()),
            ingredient: "Rye".to_owned(),
            pack_grams: Some(500.0),
            price_pence: Some(269),
            notes: Some("Rye, Organic Wholegrain — back in stock".to_owned()),
        },
        Source {
            supplier: "Hodmedod's".to_owned(),
            url: Some("https://hodmedods.co.uk/products/naked-oats-organic-wholegrain".to_owned()),
            ingredient: "Naked Oats".to_owned(),
            pack_grams: Some(500.0),
            price_pence: Some(199),
            notes: Some("Naked Oats, Organic Wholegrain — non-GF (mixing with wheat anyway)".to_owned()),
        },
        Source {
            supplier: "Forest Fungi".to_owned(),
            url: Some("https://forestfungi.co.uk/product/dried-porcini-mushrooms/".to_owned()),
            ingredient: "Porcini Mushrooms".to_owned(),
            pack_grams: Some(50.0),
            price_pence: Some(895),
            notes: Some("British-grown in Devon — closest supplier to Cornwall".to_owned()),
        },
    ]
}

pub fn skog_grautr_processes() -> Vec<Process> {
    vec![
        Process {
            input: "Wheat".to_owned(),
            output_form: Form::Cracked,
            method: ProcessMethod::HandMill,
            notes: Some("Zassenhaus on coarse setting — 2-3 pieces per kernel".to_owned()),
        },
        Process {
            input: "Rye".to_owned(),
            output_form: Form::Cracked,
            method: ProcessMethod::HandMill,
            notes: Some("Zassenhaus on coarse setting — rye is dense, takes more turns".to_owned()),
        },
        Process {
            input: "Naked Oats".to_owned(),
            output_form: Form::Cracked,
            method: ProcessMethod::HandMill,
            notes: Some("Zassenhaus on coarse setting — soft grain, easiest to crack".to_owned()),
        },
        Process {
            input: "Onion".to_owned(),
            output_form: Form::Dehydrated,
            method: ProcessMethod::Dehydrate { temp_c: 57, hours: 8.0 },
            notes: Some("Ninja Foodi dehydrate function. Slice thin, until crisp, then crumble.".to_owned()),
        },
        Process {
            input: "Porcini Mushrooms".to_owned(),
            output_form: Form::Crumbled,
            method: ProcessMethod::Crumble,
            notes: Some("Break dried porcini into small pieces by hand".to_owned()),
        },
    ]
}

pub fn skog_grautr_mix() -> Mix {
    Mix {
        name: "Skógargrautr".to_owned(),
        description: "Forest porridge — cracked Nordic grains with wild porcini, \
            parsley and dried onion. Old Norse skógar (of the forest) + grautr (porridge)."
            .to_owned(),
        components: vec![
            MixComponent {
                ingredient: "Wheat".to_owned(),
                form: Form::Cracked,
                proportion: Proportion::EqualPart,
            },
            MixComponent {
                ingredient: "Naked Barley".to_owned(),
                form: Form::Kibbled,
                proportion: Proportion::EqualPart,
            },
            MixComponent {
                ingredient: "Rye".to_owned(),
                form: Form::Cracked,
                proportion: Proportion::EqualPart,
            },
            MixComponent {
                ingredient: "Naked Oats".to_owned(),
                form: Form::Cracked,
                proportion: Proportion::EqualPart,
            },
            MixComponent {
                ingredient: "Porcini Mushrooms".to_owned(),
                form: Form::Crumbled,
                proportion: Proportion::ToTaste,
            },
            MixComponent {
                ingredient: "Parsley".to_owned(),
                form: Form::Dried,
                proportion: Proportion::ToTaste,
            },
            MixComponent {
                ingredient: "Onion".to_owned(),
                form: Form::Dehydrated,
                proportion: Proportion::ToTaste,
            },
        ],
        container: Some("Muji Heat Proof Jar 800ml A".to_owned()),
        yield_grams: None,
    }
}

pub fn skog_grautr_recipe() -> Recipe {
    Recipe {
        name: "Skógargrautr".to_owned(),
        mix: "Skógargrautr".to_owned(),
        servings: 1,
        steps: vec![
            CookingStep {
                instruction: "Pour 250ml water into a small saucepan".to_owned(),
                duration_minutes: None,
                water_ml: Some(250),
                heat: None,
            },
            CookingStep {
                instruction: "Add one serving of Skógargrautr mix".to_owned(),
                duration_minutes: None,
                water_ml: None,
                heat: None,
            },
            CookingStep {
                instruction: "Bring to a boil".to_owned(),
                duration_minutes: None,
                water_ml: None,
                heat: Some(HeatLevel::Boil),
            },
            CookingStep {
                instruction: "Remove from heat, close lid, and rest".to_owned(),
                duration_minutes: Some(10.0),
                water_ml: None,
                heat: Some(HeatLevel::Off),
            },
            CookingStep {
                instruction: "Serve".to_owned(),
                duration_minutes: None,
                water_ml: None,
                heat: None,
            },
        ],
    }
}

pub fn muji_containers() -> Vec<Container> {
    vec![
        // 4x large glass jars
        Container {
            name: "Muji Heat Proof Jar 800ml A".to_owned(),
            kind: ContainerKind::MujiHeatProofJar,
            capacity_ml: 800,
        },
        Container {
            name: "Muji Heat Proof Jar 800ml B".to_owned(),
            kind: ContainerKind::MujiHeatProofJar,
            capacity_ml: 800,
        },
        Container {
            name: "Muji Heat Proof Jar 800ml C".to_owned(),
            kind: ContainerKind::MujiHeatProofJar,
            capacity_ml: 800,
        },
        Container {
            name: "Muji Heat Proof Jar 800ml D".to_owned(),
            kind: ContainerKind::MujiHeatProofJar,
            capacity_ml: 800,
        },
        // 2x medium glass jars
        Container {
            name: "Muji Heat Proof Jar 500ml A".to_owned(),
            kind: ContainerKind::MujiHeatProofJar,
            capacity_ml: 500,
        },
        Container {
            name: "Muji Heat Proof Jar 500ml B".to_owned(),
            kind: ContainerKind::MujiHeatProofJar,
            capacity_ml: 500,
        },
        // 4x medium tubs
        Container {
            name: "Muji Storage Container 1.5L A".to_owned(),
            kind: ContainerKind::MujiStorageContainer,
            capacity_ml: 1500,
        },
        Container {
            name: "Muji Storage Container 1.5L B".to_owned(),
            kind: ContainerKind::MujiStorageContainer,
            capacity_ml: 1500,
        },
        Container {
            name: "Muji Storage Container 1.5L C".to_owned(),
            kind: ContainerKind::MujiStorageContainer,
            capacity_ml: 1500,
        },
        Container {
            name: "Muji Storage Container 1.5L D".to_owned(),
            kind: ContainerKind::MujiStorageContainer,
            capacity_ml: 1500,
        },
        // 1x short tub
        Container {
            name: "Muji Storage Container 710ml A".to_owned(),
            kind: ContainerKind::MujiStorageContainer,
            capacity_ml: 710,
        },
    ]
}

/// Raw ingredient storage — what goes where before processing
pub fn raw_storage_mixes() -> Vec<Mix> {
    vec![
        // === 800ml jars ===
        // A = Skógargrautr mix (assigned on the mix itself)
        Mix {
            name: "Dried Fava Beans (stock)".to_owned(),
            description: "Whole dried field beans — soak overnight before cooking".to_owned(),
            components: vec![MixComponent {
                ingredient: "Fava Beans".to_owned(),
                form: Form::Dried,
                proportion: Proportion::Grams(500.0),
            }],
            container: Some("Muji Heat Proof Jar 800ml B".to_owned()),
            yield_grams: Some(500.0),
        },
        Mix {
            name: "Dried Parsley (stock)".to_owned(),
            description: "Dried parsley flakes".to_owned(),
            components: vec![MixComponent {
                ingredient: "Parsley".to_owned(),
                form: Form::Dried,
                proportion: Proportion::ToTaste,
            }],
            container: Some("Muji Heat Proof Jar 800ml C".to_owned()),
            yield_grams: None,
        },
        // D = free
        // === 500ml jars ===
        Mix {
            name: "Dried Porcini (stock)".to_owned(),
            description: "Whole dried porcini slices from Forest Fungi, Devon".to_owned(),
            components: vec![MixComponent {
                ingredient: "Porcini Mushrooms".to_owned(),
                form: Form::Dried,
                proportion: Proportion::Grams(50.0),
            }],
            container: Some("Muji Heat Proof Jar 500ml A".to_owned()),
            yield_grams: Some(50.0),
        },
        Mix {
            name: "Dehydrated Onion (stock)".to_owned(),
            description: "Home-dehydrated in Ninja Foodi, crumbled".to_owned(),
            components: vec![MixComponent {
                ingredient: "Onion".to_owned(),
                form: Form::Dehydrated,
                proportion: Proportion::ToTaste,
            }],
            container: Some("Muji Heat Proof Jar 500ml B".to_owned()),
            yield_grams: None,
        },
        // === 1.5L tubs ===
        Mix {
            name: "Whole Wheat (raw stock)".to_owned(),
            description: "YQ Wheat berries, whole, awaiting cracking".to_owned(),
            components: vec![MixComponent {
                ingredient: "Wheat".to_owned(),
                form: Form::Whole,
                proportion: Proportion::Grams(500.0),
            }],
            container: Some("Muji Storage Container 1.5L A".to_owned()),
            yield_grams: Some(500.0),
        },
        Mix {
            name: "Whole Rye (raw stock)".to_owned(),
            description: "Organic rye grain, whole, awaiting cracking".to_owned(),
            components: vec![MixComponent {
                ingredient: "Rye".to_owned(),
                form: Form::Whole,
                proportion: Proportion::Grams(500.0),
            }],
            container: Some("Muji Storage Container 1.5L B".to_owned()),
            yield_grams: Some(500.0),
        },
        Mix {
            name: "Naked Oat Groats (raw stock)".to_owned(),
            description: "Whole naked oats, awaiting cracking. Store cool.".to_owned(),
            components: vec![MixComponent {
                ingredient: "Naked Oats".to_owned(),
                form: Form::Whole,
                proportion: Proportion::Grams(500.0),
            }],
            container: Some("Muji Storage Container 1.5L C".to_owned()),
            yield_grams: Some(500.0),
        },
        Mix {
            name: "Whole Emmer (raw stock)".to_owned(),
            description: "British emmer berries, whole, awaiting cracking in Zassenhaus".to_owned(),
            components: vec![MixComponent {
                ingredient: "Emmer".to_owned(),
                form: Form::Whole,
                proportion: Proportion::Grams(500.0),
            }],
            container: Some("Muji Storage Container 1.5L D".to_owned()),
            yield_grams: Some(500.0),
        },
        // === 710ml tub ===
        Mix {
            name: "Kibbled Naked Barley (ready)".to_owned(),
            description: "Pre-kibbled by Hodmedod's — ready to use directly in mix".to_owned(),
            components: vec![MixComponent {
                ingredient: "Naked Barley".to_owned(),
                form: Form::Kibbled,
                proportion: Proportion::Grams(500.0),
            }],
            container: Some("Muji Storage Container 710ml A".to_owned()),
            yield_grams: Some(500.0),
        },
    ]
}

// ============================================================
// PULS FABATA — Republic-era bean and grain porridge
// ============================================================

pub fn puls_fabata_ingredients() -> Vec<Ingredient> {
    vec![
        Ingredient {
            name: "Emmer".to_owned(),
            kind: IngredientKind::Grain,
            form: Form::Whole,
            notes: Some(
                "Far/farro — the grain that built Rome. British-grown by Jeremy Dickin, Lincolnshire."
                    .to_owned(),
            ),
        },
        Ingredient {
            name: "Fava Beans".to_owned(),
            kind: IngredientKind::Legume,
            form: Form::Dried,
            notes: Some(
                "Whole dried field beans (Vicia faba minor) — small, dense, brown. \
                Not broad beans. Grown on the Moray Firth, Scotland."
                    .to_owned(),
            ),
        },
        Ingredient {
            name: "Pork Fat".to_owned(),
            kind: IngredientKind::Fat,
            form: Form::Whole,
            notes: Some(
                "Guanciale, pancetta, or salt pork. Lardum — the fat of the common Roman. \
                From the butcher."
                    .to_owned(),
            ),
        },
        Ingredient {
            name: "Olive Oil".to_owned(),
            kind: IngredientKind::Fat,
            form: Form::Whole,
            notes: Some("For finishing. The Romans drizzled it on top.".to_owned()),
        },
        Ingredient {
            name: "Salt".to_owned(),
            kind: IngredientKind::Seasoning,
            form: Form::Whole,
            notes: Some("The one non-negotiable seasoning.".to_owned()),
        },
    ]
}

pub fn puls_fabata_sources() -> Vec<Source> {
    vec![
        Source {
            supplier: "Hodmedod's".to_owned(),
            url: Some("https://hodmedods.co.uk/products/emmer-grain".to_owned()),
            ingredient: "Emmer".to_owned(),
            pack_grams: Some(500.0),
            price_pence: Some(299),
            notes: Some("Emmer (Farro), Wholegrain — British-grown in Lincolnshire".to_owned()),
        },
        Source {
            supplier: "Hodmedod's".to_owned(),
            url: Some("https://hodmedods.co.uk/products/whole-fava-beans".to_owned()),
            ingredient: "Fava Beans".to_owned(),
            pack_grams: Some(500.0),
            price_pence: Some(229),
            notes: Some("Whole Fava Beans, Organic — field beans, brown and round".to_owned()),
        },
        Source {
            supplier: "Local butcher".to_owned(),
            url: None,
            ingredient: "Pork Fat".to_owned(),
            pack_grams: None,
            price_pence: None,
            notes: Some("Guanciale, pancetta, or salt pork. Ask for pork jowl or belly offcuts.".to_owned()),
        },
    ]
}

pub fn puls_fabata_processes() -> Vec<Process> {
    vec![Process {
        input: "Emmer".to_owned(),
        output_form: Form::Cracked,
        method: ProcessMethod::HandMill,
        notes: Some(
            "Coarse crack in Zassenhaus. The Romans called this alica — \
            pounded emmer. Not flour, just broken kernels."
                .to_owned(),
        ),
    }]
}

pub fn puls_fabata_mix() -> Mix {
    Mix {
        name: "Puls Fabata".to_owned(),
        description: "Republic-era bean and grain porridge. Pallens faba cum rubente lardo — \
            pale beans with red bacon. The food that built Rome."
            .to_owned(),
        components: vec![
            MixComponent {
                ingredient: "Emmer".to_owned(),
                form: Form::Cracked,
                proportion: Proportion::Grams(250.0),
            },
            MixComponent {
                ingredient: "Fava Beans".to_owned(),
                form: Form::Dried,
                proportion: Proportion::Grams(150.0),
            },
            MixComponent {
                ingredient: "Pork Fat".to_owned(),
                form: Form::Whole,
                proportion: Proportion::Grams(100.0),
            },
            MixComponent {
                ingredient: "Onion".to_owned(),
                form: Form::Whole,
                proportion: Proportion::Grams(100.0),
            },
            MixComponent {
                ingredient: "Olive Oil".to_owned(),
                form: Form::Whole,
                proportion: Proportion::ToTaste,
            },
            MixComponent {
                ingredient: "Salt".to_owned(),
                form: Form::Whole,
                proportion: Proportion::ToTaste,
            },
        ],
        container: None,
        yield_grams: None,
    }
}

pub fn puls_fabata_recipe() -> Recipe {
    Recipe {
        name: "Puls Fabata".to_owned(),
        mix: "Puls Fabata".to_owned(),
        servings: 2,
        steps: vec![
            CookingStep {
                instruction: "Night before: soak fava beans in cold water overnight".to_owned(),
                duration_minutes: Some(480.0),
                water_ml: None,
                heat: None,
            },
            CookingStep {
                instruction: "Night before: soak cracked emmer in water separately".to_owned(),
                duration_minutes: Some(480.0),
                water_ml: None,
                heat: None,
            },
            CookingStep {
                instruction: "Drain and rinse both beans and grain".to_owned(),
                duration_minutes: None,
                water_ml: None,
                heat: None,
            },
            CookingStep {
                instruction: "Cut pork fat into small pieces. Render slowly in a heavy pot \
                    until fat runs freely and meat begins to crisp"
                    .to_owned(),
                duration_minutes: Some(10.0),
                water_ml: None,
                heat: Some(HeatLevel::Low),
            },
            CookingStep {
                instruction: "Add chopped onion to the rendered fat, cook until softened".to_owned(),
                duration_minutes: Some(5.0),
                water_ml: None,
                heat: Some(HeatLevel::Low),
            },
            CookingStep {
                instruction: "Add drained emmer, stir through the fat. Add water to cover well"
                    .to_owned(),
                duration_minutes: None,
                water_ml: Some(1000),
                heat: None,
            },
            CookingStep {
                instruction: "Bring to a gentle simmer — not a hard boil. \
                    The more you cook it, the better will be the puls"
                    .to_owned(),
                duration_minutes: Some(20.0),
                water_ml: None,
                heat: Some(HeatLevel::Low),
            },
            CookingStep {
                instruction: "Add drained fava beans. Continue simmering, stirring regularly. \
                    Add water as needed to maintain thick porridge"
                    .to_owned(),
                duration_minutes: Some(35.0),
                water_ml: None,
                heat: Some(HeatLevel::Low),
            },
            CookingStep {
                instruction: "Season with salt. Drizzle generously with olive oil. Serve."
                    .to_owned(),
                duration_minutes: None,
                water_ml: None,
                heat: None,
            },
        ],
    }
}

// ============================================================
// THE LARDER — everything together
// ============================================================

pub fn full_larder() -> Larder {
    let mut ingredients = skog_grautr_ingredients();
    ingredients.extend(puls_fabata_ingredients());

    let mut sources = skog_grautr_sources();
    sources.extend(puls_fabata_sources());

    let mut processes = skog_grautr_processes();
    processes.extend(puls_fabata_processes());

    let mut mixes = vec![skog_grautr_mix(), puls_fabata_mix()];
    mixes.extend(raw_storage_mixes());

    Larder {
        ingredients,
        sources,
        processes,
        containers: muji_containers(),
        mixes,
        recipes: vec![skog_grautr_recipe(), puls_fabata_recipe()],
    }
}

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
            supplier: "Merry Hill Mushrooms".to_owned(),
            url: Some("https://www.merryhill-mushrooms.co.uk/dried-porcini-500".to_owned()),
            ingredient: "Porcini Mushrooms".to_owned(),
            pack_grams: Some(250.0),
            price_pence: Some(1900),
            notes: Some("Dried Premium Porcini 250g \u{2014} matched to grain supply (~30-40 servings)".to_owned()),
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
                instruction: "Pour water into a small saucepan \u{2014} \
                    80ml for 80g serving (with protein), 125ml for 125g standalone"
                    .to_owned(),
                duration_minutes: None,
                water_ml: Some(125),
                heat: None,
            },
            CookingStep {
                instruction: "Add Skógargrautr mix \u{2014} \
                    80g when pairing with salmon/eggs, 125g standalone"
                    .to_owned(),
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
            description: "Dried premium porcini slices from Merry Hill Mushrooms".to_owned(),
            components: vec![MixComponent {
                ingredient: "Porcini Mushrooms".to_owned(),
                form: Form::Dried,
                proportion: Proportion::Grams(250.0),
            }],
            container: Some("Muji Heat Proof Jar 500ml A".to_owned()),
            yield_grams: Some(250.0),
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
    vec![
        Process {
            input: "Emmer".to_owned(),
            output_form: Form::Cracked,
            method: ProcessMethod::HandMill,
            notes: Some(
                "Coarse crack in Zassenhaus. The Romans called this alica \u{2014} \
                pounded emmer. Not flour, just broken kernels."
                    .to_owned(),
            ),
        },
        Process {
            input: "Fava Beans".to_owned(),
            output_form: Form::Cracked,
            method: ProcessMethod::HandMill,
            notes: Some(
                "Coarse crack in Zassenhaus \u{2014} breaks the hulls. \
                Field bean hulls never soften otherwise, no matter how long you cook or soak."
                    .to_owned(),
            ),
        },
    ]
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
                form: Form::Cracked,
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
                instruction: "Crack emmer and fava beans in Zassenhaus on coarse setting \u{2014} \
                    break the hulls, not flour. Everything through the mill."
                    .to_owned(),
                duration_minutes: Some(10.0),
                water_ml: None,
                heat: None,
            },
            CookingStep {
                instruction: "Everything in the pot cold: cracked beans, cracked emmer, \
                    pork fat in small pieces, chopped onion. Add water to cover well."
                    .to_owned(),
                duration_minutes: None,
                water_ml: Some(1000),
                heat: None,
            },
            CookingStep {
                instruction: "Bring to a full boil \u{2014} \
                    this destroys the lectins in the beans. Skim any foam."
                    .to_owned(),
                duration_minutes: Some(10.0),
                water_ml: None,
                heat: Some(HeatLevel::Boil),
            },
            CookingStep {
                instruction: "Drop to low. Simmer gently, stirring occasionally. \
                    Add water as needed. The fat renders into the liquid as it cooks."
                    .to_owned(),
                duration_minutes: Some(90.0),
                water_ml: None,
                heat: Some(HeatLevel::Low),
            },
            CookingStep {
                instruction: "Mash with a ladle against the side of the pot as it thickens \u{2014} \
                    maccare. The beans and grain break down into a thick porridge."
                    .to_owned(),
                duration_minutes: Some(30.0),
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
// FLATBRAUÐ — Norse unleavened flatbread
// ============================================================

// No new ingredients — uses Rye (from Skógargrautr) and Salt (from Puls Fabata)
// No new sources — rye already sourced from Hodmedod's

pub fn flatbraud_processes() -> Vec<Process> {
    vec![Process {
        input: "Rye".to_owned(),
        output_form: Form::Ground,
        method: ProcessMethod::HandMill,
        notes: Some(
            "Zassenhaus on fine setting \u{2014} grind to flour fresh before use, \
            like coffee. The volatile oils matter."
                .to_owned(),
        ),
    }]
}

pub fn flatbraud_mix() -> Mix {
    Mix {
        name: "Flatbrauð".to_owned(),
        description: "Norse rye flatbread \u{2014} the most fundamental bread. \
            Ground grain, water, salt, hot iron. No yeast, no soda, no waiting."
            .to_owned(),
        components: vec![
            MixComponent {
                ingredient: "Rye".to_owned(),
                form: Form::Ground,
                proportion: Proportion::Grams(100.0),
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

pub fn flatbraud_recipe() -> Recipe {
    Recipe {
        name: "Flatbrauð".to_owned(),
        mix: "Flatbrauð".to_owned(),
        servings: 2,
        steps: vec![
            CookingStep {
                instruction: "Grind rye in Zassenhaus on fine setting \u{2014} \
                    fresh, right before mixing"
                    .to_owned(),
                duration_minutes: Some(5.0),
                water_ml: None,
                heat: None,
            },
            CookingStep {
                instruction: "Mix ground rye with a pinch of salt. \
                    Add water gradually, knead into a firm dough"
                    .to_owned(),
                duration_minutes: None,
                water_ml: Some(60),
                heat: None,
            },
            CookingStep {
                instruction: "Divide into two pieces. Roll or press each \
                    thin on a floured surface"
                    .to_owned(),
                duration_minutes: None,
                water_ml: None,
                heat: None,
            },
            CookingStep {
                instruction: "Cook on a dry, hot cast iron pan \u{2014} \
                    no oil needed. When bubbles form and underside chars slightly, flip"
                    .to_owned(),
                duration_minutes: Some(2.0),
                water_ml: None,
                heat: Some(HeatLevel::Medium),
            },
            CookingStep {
                instruction: "Cook second side until done. Eat warm."
                    .to_owned(),
                duration_minutes: Some(2.0),
                water_ml: None,
                heat: Some(HeatLevel::Medium),
            },
        ],
    }
}

// ============================================================
// LIBUM — Cato's sacred cheese bread (De Agri Cultura, 75)
// ============================================================

pub fn libum_ingredients() -> Vec<Ingredient> {
    vec![
        Ingredient {
            name: "Fresh Cheese".to_owned(),
            kind: IngredientKind::Dairy,
            form: Form::Whole,
            notes: Some(
                "Ricotta or similar soft fresh cheese. Cato says to pound it \
                smooth in a mortar."
                    .to_owned(),
            ),
        },
        Ingredient {
            name: "Egg".to_owned(),
            kind: IngredientKind::Egg,
            form: Form::Whole,
            notes: None,
        },
        Ingredient {
            name: "Bay Leaves".to_owned(),
            kind: IngredientKind::Herb,
            form: Form::Dried,
            notes: Some("Placed underneath while baking \u{2014} aromatic bed, not eaten.".to_owned()),
        },
    ]
}

pub fn libum_sources() -> Vec<Source> {
    vec![
        Source {
            supplier: "Any shop".to_owned(),
            url: None,
            ingredient: "Fresh Cheese".to_owned(),
            pack_grams: Some(250.0),
            price_pence: None,
            notes: Some("Ricotta or any soft, fresh cheese".to_owned()),
        },
        Source {
            supplier: "Any shop".to_owned(),
            url: None,
            ingredient: "Egg".to_owned(),
            pack_grams: None,
            price_pence: None,
            notes: Some("Free range".to_owned()),
        },
        Source {
            supplier: "Any shop".to_owned(),
            url: None,
            ingredient: "Bay Leaves".to_owned(),
            pack_grams: None,
            price_pence: None,
            notes: Some("Dried bay leaves".to_owned()),
        },
    ]
}

pub fn libum_processes() -> Vec<Process> {
    vec![Process {
        input: "Emmer".to_owned(),
        output_form: Form::Ground,
        method: ProcessMethod::HandMill,
        notes: Some(
            "Zassenhaus on fine setting \u{2014} grind to flour. \
            Cato calls this far or farina."
                .to_owned(),
        ),
    }]
}

pub fn libum_mix() -> Mix {
    Mix {
        name: "Libum".to_owned(),
        description: "Cato's cheese bread (De Agri Cultura 75). Libum hoc modo facito \u{2014} \
            make libum thus. Offered to the gods, eaten by the living."
            .to_owned(),
        components: vec![
            MixComponent {
                ingredient: "Emmer".to_owned(),
                form: Form::Ground,
                proportion: Proportion::Grams(100.0),
            },
            MixComponent {
                ingredient: "Fresh Cheese".to_owned(),
                form: Form::Whole,
                proportion: Proportion::Grams(200.0),
            },
            MixComponent {
                ingredient: "Egg".to_owned(),
                form: Form::Whole,
                proportion: Proportion::Grams(50.0),
            },
            MixComponent {
                ingredient: "Bay Leaves".to_owned(),
                form: Form::Dried,
                proportion: Proportion::ToTaste,
            },
        ],
        container: None,
        yield_grams: None,
    }
}

pub fn libum_recipe() -> Recipe {
    Recipe {
        name: "Libum".to_owned(),
        mix: "Libum".to_owned(),
        servings: 1,
        steps: vec![
            CookingStep {
                instruction: "Grind emmer in Zassenhaus on fine setting \u{2014} \
                    fresh flour, right before mixing"
                    .to_owned(),
                duration_minutes: Some(5.0),
                water_ml: None,
                heat: None,
            },
            CookingStep {
                instruction: "Pound fresh cheese smooth in mortar \u{2014} \
                    Cato: casei bene disterat in mortario"
                    .to_owned(),
                duration_minutes: Some(3.0),
                water_ml: None,
                heat: None,
            },
            CookingStep {
                instruction: "Mix ground emmer into the cheese. \
                    Add one egg, mix well until uniform dough"
                    .to_owned(),
                duration_minutes: None,
                water_ml: None,
                heat: None,
            },
            CookingStep {
                instruction: "Shape into a round loaf. Place on bay leaves \
                    on a baking surface"
                    .to_owned(),
                duration_minutes: None,
                water_ml: None,
                heat: None,
            },
            CookingStep {
                instruction: "Bake gently until golden \u{2014} \
                    Cato: in foco caldo sub testu coquito leniter"
                    .to_owned(),
                duration_minutes: Some(30.0),
                water_ml: None,
                heat: Some(HeatLevel::Medium),
            },
        ],
    }
}

// ============================================================
// FAT RENDERING — turning raw butcher fat into cooking fat
// ============================================================

pub fn rendering_ingredients() -> Vec<Ingredient> {
    vec![Ingredient {
        name: "Beef Fat".to_owned(),
        kind: IngredientKind::Fat,
        form: Form::Whole,
        notes: Some(
            "Raw beef fat trimmings from the butcher \u{2014} sliced off the cuts, \
            given away free because nobody wants it. Renders into dripping."
                .to_owned(),
        ),
    }]
}

pub fn rendering_sources() -> Vec<Source> {
    vec![
        Source {
            supplier: "Local butcher".to_owned(),
            url: None,
            ingredient: "Pork Fat".to_owned(),
            pack_grams: None,
            price_pence: Some(0),
            notes: Some(
                "Free raw pork fat trimmings \u{2014} off the cuts, for rendering into lard"
                    .to_owned(),
            ),
        },
        Source {
            supplier: "Local butcher".to_owned(),
            url: None,
            ingredient: "Beef Fat".to_owned(),
            pack_grams: None,
            price_pence: Some(0),
            notes: Some(
                "Free raw beef fat trimmings \u{2014} off the cuts, for rendering into dripping"
                    .to_owned(),
            ),
        },
    ]
}

pub fn rendering_processes() -> Vec<Process> {
    vec![
        Process {
            input: "Pork Fat".to_owned(),
            output_form: Form::Rendered,
            method: ProcessMethod::Render,
            notes: Some(
                "Cut into small pieces. Low heat in a heavy pot until fat runs clear. \
                Strain through muslin. Cool and store. This is lard."
                    .to_owned(),
            ),
        },
        Process {
            input: "Beef Fat".to_owned(),
            output_form: Form::Rendered,
            method: ProcessMethod::Render,
            notes: Some(
                "Cut into small pieces. Low heat in a heavy pot until fat runs clear. \
                Strain through muslin. Cool and store. This is dripping."
                    .to_owned(),
            ),
        },
    ]
}

// ============================================================
// THE LARDER — everything together
// ============================================================

pub fn full_larder() -> Larder {
    let mut ingredients = skog_grautr_ingredients();
    ingredients.extend(puls_fabata_ingredients());
    ingredients.extend(libum_ingredients());
    ingredients.extend(rendering_ingredients());

    let mut sources = skog_grautr_sources();
    sources.extend(puls_fabata_sources());
    sources.extend(libum_sources());
    sources.extend(rendering_sources());

    let mut processes = skog_grautr_processes();
    processes.extend(puls_fabata_processes());
    processes.extend(flatbraud_processes());
    processes.extend(libum_processes());
    processes.extend(rendering_processes());

    let mut mixes = vec![
        skog_grautr_mix(),
        puls_fabata_mix(),
        flatbraud_mix(),
        libum_mix(),
    ];
    mixes.extend(raw_storage_mixes());

    Larder {
        ingredients,
        sources,
        processes,
        containers: muji_containers(),
        mixes,
        recipes: vec![
            skog_grautr_recipe(),
            puls_fabata_recipe(),
            flatbraud_recipe(),
            libum_recipe(),
        ],
    }
}

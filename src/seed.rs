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
            notes: Some("Hulless oats, grown in Scotland. Store cold — beneficial oils.".to_owned()),
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
            notes: Some("YQ Wheat, Organic Wholegrain".to_owned()),
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
            url: Some("https://hodmedods.co.uk/products/naked-oat-groats-gluten-free-organic".to_owned()),
            ingredient: "Naked Oats".to_owned(),
            pack_grams: Some(1000.0),
            price_pence: Some(466),
            notes: Some("Naked Oat Groats, GF, Organic — 500g sold out, order 1kg".to_owned()),
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
        name: "Skog Grautr".to_owned(),
        description: "Forest porridge — cracked Nordic grains with wild porcini, \
            parsley and dried onion. Old Norse skog (forest) + grautr (porridge)."
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
        name: "Skog Grautr".to_owned(),
        mix: "Skog Grautr".to_owned(),
        servings: 1,
        steps: vec![
            CookingStep {
                instruction: "Pour 250ml water into a small saucepan".to_owned(),
                duration_minutes: None,
                water_ml: Some(250),
                heat: None,
            },
            CookingStep {
                instruction: "Add one serving of Skog Grautr mix".to_owned(),
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
        // === Assigned ===
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
            name: "Muji Heat Proof Jar 500ml A".to_owned(),
            kind: ContainerKind::MujiHeatProofJar,
            capacity_ml: 500,
        },
        Container {
            name: "Muji Heat Proof Jar 500ml B".to_owned(),
            kind: ContainerKind::MujiHeatProofJar,
            capacity_ml: 500,
        },
        Container {
            name: "Muji Heat Proof Jar 500ml C".to_owned(),
            kind: ContainerKind::MujiHeatProofJar,
            capacity_ml: 500,
        },
        Container {
            name: "Muji Storage Container 710ml A".to_owned(),
            kind: ContainerKind::MujiStorageContainer,
            capacity_ml: 710,
        },
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
        // === Unassigned ===
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
        Container {
            name: "Muji Heat Proof Jar 500ml D".to_owned(),
            kind: ContainerKind::MujiHeatProofJar,
            capacity_ml: 500,
        },
        Container {
            name: "Muji Storage Container 710ml B".to_owned(),
            kind: ContainerKind::MujiStorageContainer,
            capacity_ml: 710,
        },
    ]
}

/// Raw grain storage — what goes where before processing
pub fn raw_storage_mixes() -> Vec<Mix> {
    vec![
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
                proportion: Proportion::Grams(1000.0),
            }],
            container: Some("Muji Heat Proof Jar 800ml B".to_owned()),
            yield_grams: Some(1000.0),
        },
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
        Mix {
            name: "Dried Parsley (stock)".to_owned(),
            description: "Dried parsley flakes".to_owned(),
            components: vec![MixComponent {
                ingredient: "Parsley".to_owned(),
                form: Form::Dried,
                proportion: Proportion::ToTaste,
            }],
            container: Some("Muji Heat Proof Jar 500ml C".to_owned()),
            yield_grams: None,
        },
    ]
}

pub fn skog_grautr_larder() -> Larder {
    let mut mixes = vec![skog_grautr_mix()];
    mixes.extend(raw_storage_mixes());

    Larder {
        ingredients: skog_grautr_ingredients(),
        sources: skog_grautr_sources(),
        processes: skog_grautr_processes(),
        containers: muji_containers(),
        mixes,
        recipes: vec![skog_grautr_recipe()],
    }
}

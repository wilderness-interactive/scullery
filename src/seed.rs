use crate::data::*;

pub fn skog_grautr_ingredients() -> Vec<Ingredient> {
    vec![
        Ingredient {
            name: "Wheat".to_owned(),
            kind: IngredientKind::Grain,
            form: Form::Whole,
            notes: Some("British-grown whole wheat berries from Hodmedod's".to_owned()),
        },
        Ingredient {
            name: "Naked Barley".to_owned(),
            kind: IngredientKind::Grain,
            form: Form::Whole,
            notes: Some("Hulless barley — the ancient form, no pearling needed".to_owned()),
        },
        Ingredient {
            name: "Rye".to_owned(),
            kind: IngredientKind::Grain,
            form: Form::Whole,
            notes: Some("British-grown whole rye from Hodmedod's".to_owned()),
        },
        Ingredient {
            name: "Naked Oats".to_owned(),
            kind: IngredientKind::Grain,
            form: Form::Whole,
            notes: Some("Hulless oats — hull falls away naturally, whole unrefined".to_owned()),
        },
        Ingredient {
            name: "Porcini Mushrooms".to_owned(),
            kind: IngredientKind::Mushroom,
            form: Form::Dried,
            notes: Some("Dried porcini, the deep umami earthiness of the mix".to_owned()),
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
            notes: Some("Home-dehydrated — shop-bought onion powder is bitter and acrid".to_owned()),
        },
    ]
}

pub fn skog_grautr_sources() -> Vec<Source> {
    vec![
        Source {
            supplier: "Hodmedod's".to_owned(),
            url: Some("https://hodmedods.co.uk/collections/grains".to_owned()),
            ingredient: "Wheat".to_owned(),
            pack_grams: Some(500.0),
            price_pence: None,
            notes: Some("YQ Wheat or Flanders Wheat, wholegrain".to_owned()),
        },
        Source {
            supplier: "Hodmedod's".to_owned(),
            url: Some("https://hodmedods.co.uk/collections/grains".to_owned()),
            ingredient: "Naked Barley".to_owned(),
            pack_grams: Some(500.0),
            price_pence: None,
            notes: Some("Naked barley, wholegrain".to_owned()),
        },
        Source {
            supplier: "Hodmedod's".to_owned(),
            url: Some("https://hodmedods.co.uk/collections/grains".to_owned()),
            ingredient: "Rye".to_owned(),
            pack_grams: Some(500.0),
            price_pence: None,
            notes: Some("Organic wholegrain rye — check availability, sometimes out of stock".to_owned()),
        },
        Source {
            supplier: "Hodmedod's".to_owned(),
            url: Some("https://hodmedods.co.uk/collections/grains".to_owned()),
            ingredient: "Naked Oats".to_owned(),
            pack_grams: Some(500.0),
            price_pence: None,
            notes: Some("Naked oat groats, gluten-free, organic".to_owned()),
        },
    ]
}

pub fn skog_grautr_processes() -> Vec<Process> {
    vec![
        Process {
            input: "Wheat".to_owned(),
            output_form: Form::Cracked,
            method: ProcessMethod::MortarAndPestle,
            notes: Some("Coarse crack — not flour, not flakes. Two or three pieces per kernel.".to_owned()),
        },
        Process {
            input: "Naked Barley".to_owned(),
            output_form: Form::Kibbled,
            method: ProcessMethod::MortarAndPestle,
            notes: Some("Kibble to rough irregular pieces".to_owned()),
        },
        Process {
            input: "Rye".to_owned(),
            output_form: Form::Cracked,
            method: ProcessMethod::MortarAndPestle,
            notes: Some("Coarse crack — rye is dense, takes more work than oats".to_owned()),
        },
        Process {
            input: "Naked Oats".to_owned(),
            output_form: Form::Cracked,
            method: ProcessMethod::MortarAndPestle,
            notes: Some("Soft grain, cracks easily. A few firm strikes per handful.".to_owned()),
        },
        Process {
            input: "Onion".to_owned(),
            output_form: Form::Dehydrated,
            method: ProcessMethod::Dehydrate { temp_c: 57, hours: 8.0 },
            notes: Some("Slice thin, dehydrate until crisp, then crumble into mix".to_owned()),
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
            name: "Muji Heat Proof Jar 500ml D".to_owned(),
            kind: ContainerKind::MujiHeatProofJar,
            capacity_ml: 500,
        },
        Container {
            name: "Muji Storage Container 710ml A".to_owned(),
            kind: ContainerKind::MujiStorageContainer,
            capacity_ml: 710,
        },
        Container {
            name: "Muji Storage Container 710ml B".to_owned(),
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
    ]
}

pub fn skog_grautr_larder() -> Larder {
    Larder {
        ingredients: skog_grautr_ingredients(),
        sources: skog_grautr_sources(),
        processes: skog_grautr_processes(),
        containers: muji_containers(),
        mixes: vec![skog_grautr_mix()],
        recipes: vec![skog_grautr_recipe()],
    }
}

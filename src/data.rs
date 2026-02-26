use serde::{Deserialize, Serialize};

// === Physical form of any dry ingredient ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Form {
    Whole,
    Cracked,
    Kibbled,
    Flaked,
    Pearled,
    Dried,
    Dehydrated,
    Crumbled,
    Ground,
}

// === Ingredients ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IngredientKind {
    Grain,
    Mushroom,
    Herb,
    Allium,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub kind: IngredientKind,
    pub form: Form,
    pub notes: Option<String>,
}

// === Sourcing ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub supplier: String,
    pub url: Option<String>,
    pub ingredient: String,
    pub pack_grams: Option<f64>,
    pub price_pence: Option<u32>,
    pub notes: Option<String>,
}

// === Processing ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessMethod {
    MortarAndPestle,
    HandMill,
    Dehydrate { temp_c: u32, hours: f64 },
    Crumble,
    SliceAndDry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    pub input: String,
    pub output_form: Form,
    pub method: ProcessMethod,
    pub notes: Option<String>,
}

// === Containers ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerKind {
    MujiHeatProofJar,
    MujiStorageContainer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub kind: ContainerKind,
    pub capacity_ml: u32,
}

// === Mixes ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Proportion {
    EqualPart,
    Grams(f64),
    Tablespoons(f64),
    Teaspoons(f64),
    ToTaste,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixComponent {
    pub ingredient: String,
    pub form: Form,
    pub proportion: Proportion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mix {
    pub name: String,
    pub description: String,
    pub components: Vec<MixComponent>,
    pub container: Option<String>,
    pub yield_grams: Option<f64>,
}

// === Cooking ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeatLevel {
    Off,
    Low,
    Medium,
    High,
    Boil,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookingStep {
    pub instruction: String,
    pub duration_minutes: Option<f64>,
    pub water_ml: Option<u32>,
    pub heat: Option<HeatLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub mix: String,
    pub servings: u32,
    pub steps: Vec<CookingStep>,
}

// === The Larder: where all provisions live ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Larder {
    pub ingredients: Vec<Ingredient>,
    pub sources: Vec<Source>,
    pub processes: Vec<Process>,
    pub containers: Vec<Container>,
    pub mixes: Vec<Mix>,
    pub recipes: Vec<Recipe>,
}

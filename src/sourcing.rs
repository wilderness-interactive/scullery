use crate::data::*;

pub struct ShoppingItem {
    pub ingredient: String,
    pub supplier: String,
    pub pack_grams: Option<f64>,
}

pub fn shopping_list_for_mix(larder: &Larder, mix_name: &str) -> Vec<ShoppingItem> {
    let Some(mix) = larder.mixes.iter().find(|m| m.name == mix_name) else {
        return Vec::new();
    };

    mix.components
        .iter()
        .filter_map(|component| {
            let source = larder
                .sources
                .iter()
                .find(|s| s.ingredient == component.ingredient);

            Some(ShoppingItem {
                ingredient: component.ingredient.clone(),
                supplier: source
                    .map(|s| s.supplier.clone())
                    .unwrap_or_else(|| "any wholefoods shop".to_owned()),
                pack_grams: source.and_then(|s| s.pack_grams),
            })
        })
        .collect()
}

pub fn print_shopping_list(items: &[ShoppingItem]) {
    println!("=== Shopping List ===\n");
    for item in items {
        let pack = item
            .pack_grams
            .map(|g| format!(" ({}g pack)", g))
            .unwrap_or_default();
        println!(
            "  [ ] {} -- from {}{}",
            item.ingredient, item.supplier, pack
        );
    }
    println!();
}

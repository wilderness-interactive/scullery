use crate::data::*;

#[allow(dead_code)]
pub struct ShoppingItem {
    pub ingredient: String,
    pub supplier: String,
    pub pack_grams: Option<f64>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

// === Combined order across all recipes ===

pub struct OrderItem {
    pub ingredient: String,
    pub supplier: String,
    pub product_name: Option<String>,
    pub pack_grams: Option<f64>,
    pub price_pence: Option<u32>,
}

pub fn combined_order(larder: &Larder) -> Vec<OrderItem> {
    let mut seen: Vec<String> = Vec::new();
    let mut items = Vec::new();

    for recipe in &larder.recipes {
        let Some(mix) = larder.mixes.iter().find(|m| m.name == recipe.mix) else {
            continue;
        };
        for component in &mix.components {
            if seen.contains(&component.ingredient) {
                continue;
            }
            seen.push(component.ingredient.clone());

            let source = larder
                .sources
                .iter()
                .find(|s| s.ingredient == component.ingredient);

            items.push(OrderItem {
                ingredient: component.ingredient.clone(),
                supplier: source
                    .map(|s| s.supplier.clone())
                    .unwrap_or_else(|| "Any shop".to_owned()),
                product_name: source.and_then(|s| s.notes.clone()),
                pack_grams: source.and_then(|s| s.pack_grams),
                price_pence: source.and_then(|s| s.price_pence),
            });
        }
    }

    // Sort by supplier then ingredient
    items.sort_by(|a, b| a.supplier.cmp(&b.supplier).then(a.ingredient.cmp(&b.ingredient)));
    items
}

pub fn print_combined_order(items: &[OrderItem]) {
    println!("=== Combined Order ===\n");

    // Collect unique suppliers in order
    let mut suppliers: Vec<&str> = Vec::new();
    for item in items {
        if !suppliers.contains(&item.supplier.as_str()) {
            suppliers.push(&item.supplier);
        }
    }

    let mut grand_total: u32 = 0;

    for supplier in &suppliers {
        let supplier_items: Vec<&OrderItem> =
            items.iter().filter(|i| i.supplier == *supplier).collect();
        let supplier_total: u32 = supplier_items.iter().filter_map(|i| i.price_pence).sum();

        let item_word = if supplier_items.len() == 1 { "item" } else { "items" };
        if supplier_total > 0 {
            println!(
                "--- {} ({} {}, \u{00A3}{}.{:02}) ---",
                supplier,
                supplier_items.len(),
                item_word,
                supplier_total / 100,
                supplier_total % 100,
            );
        } else {
            println!("--- {} ---", supplier);
        }

        for item in &supplier_items {
            let product = item
                .product_name
                .as_ref()
                .map(|n| format!(" \u{2014} {}", n))
                .unwrap_or_default();
            let pack = item
                .pack_grams
                .map(|g| {
                    if g >= 1000.0 {
                        format!(" ({}kg)", g / 1000.0)
                    } else {
                        format!(" ({}g)", g)
                    }
                })
                .unwrap_or_default();
            let price = item
                .price_pence
                .map(|p| format!(" \u{2014} \u{00A3}{}.{:02}", p / 100, p % 100))
                .unwrap_or_default();
            println!("  [ ] {}{}{}{}", item.ingredient, product, pack, price);
        }

        grand_total += supplier_total;
        println!();
    }

    if grand_total > 0 {
        println!(
            "Total (sourced online): \u{00A3}{}.{:02}\n",
            grand_total / 100,
            grand_total % 100,
        );
    }
}

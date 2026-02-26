use crate::data::*;

pub fn find_recipe_for_mix<'a>(larder: &'a Larder, mix_name: &str) -> Option<&'a Recipe> {
    larder.recipes.iter().find(|r| r.mix == mix_name)
}

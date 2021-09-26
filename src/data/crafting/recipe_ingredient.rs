use crate::prelude::*;

pub struct RecipeIngredient {
    //What is this part named? ex: is it a tool head, tool handle?
    pub part_name: &'static str,
    //What is required for this particular item?
    pub requirement: RecipeRequirement,
}

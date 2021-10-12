use specs::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CraftGoal {
    //Child goals and data here
    recipe: Recipe,
    ingredients: Vec<RecipeIngredient>,
}

impl AIGoalTrait for CraftGoal {
    fn get_textual_representation(&self, data: &RenderData) -> String {
        //TODO: have the string print the ingredients if they exist

        format!("Craft a {}", recipe.get_resulting_object_name())
    }

    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        //TODO: rewrite
        //This should be created fully formed from the input component

        // if let Some(inp) = inp {
        //     if let Some(inv) = inv {
        //         if let Some(recipe) = recipe {
        //             let requirements = recipe.get_ingredient_requirements();
        //             let ing_len = ingredients.len();
        //             let req_len = requirements.len();

        //             if ing_len == req_len {
        //                 //Check that all ingredients fulfill their respective requirements
        //                 act.current_action = Some(AIAction::Craft {
        //                     recipe: *recipe,
        //                     ingredients: ingredients.clone(),
        //                 });
        //             } else if ing_len < req_len {
        //                 //Ask for next ingredient
        //                 let requirement = &requirements[ing_len];

        //                 let ingredient_goals = inv
        //                     .items
        //                     .iter()
        //                     .enumerate()
        //                     .filter_map(|(i, slot)| {
        //                         slot.and_then(|item| {
        //                             if requirement
        //                                 .requirement
        //                                 .requirement_fulfilled(item, &crd)
        //                             {
        //                                 let mut appended_ingredients =
        //                                     ingredients.clone();
        //                                 appended_ingredients.push(item);

        //                                 Some(PopupListItem::from((
        //                                     i,
        //                                     if let Some(item_name) = nam.get(item) {
        //                                         Some(item_name.name.clone())
        //                                     } else {
        //                                         None
        //                                     },
        //                                     AIGoal::Craft {
        //                                         recipe: Some(*recipe),
        //                                         ingredients: appended_ingredients,
        //                                     },
        //                                 )))
        //                             } else {
        //                                 None
        //                             }
        //                         })
        //                     })
        //                     .collect();

        //                 inp.popup = Some(Popup::list(
        //                     format!("Use what for {}?", requirement.part_name),
        //                     ingredient_goals,
        //                 ));
        //             } else {
        //                 //Something is very, very wrong
        //                 println!("Entity attempting to pass too many ingredients to recipe!");
        //             }
        //         } else {
        //             //TODO: allow this to take from surrounding tiles
        //             //Maybe add a utility function to return all surrounding entities
        //             let craft_goals = Recipe::iter()
        //                 .filter(|recipe| {
        //                     recipe.fulfillable_with_inventory_contents(inv, &crd)
        //                 })
        //                 .enumerate()
        //                 .map(|(i, recipe)| {
        //                     PopupListItem::from((
        //                         i,
        //                         None,
        //                         AIGoal::Craft {
        //                             recipe: Some(recipe),
        //                             ingredients: Vec::new(),
        //                         },
        //                     ))
        //                 })
        //                 .collect();

        //             inp.popup =
        //                 Some(Popup::list(format!("Craft what?"), craft_goals));
        //         }
        //     } else {
        //         println!("Entity attempting to find recipe ingredients without an inventory!");
        //     }
        // } else {
        //     println!("Entity attempting to find recipe to craft without an input component!");
        // }
        Self::success()
    }
}

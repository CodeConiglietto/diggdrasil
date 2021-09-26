use crate::prelude::*;
use specs::Entity;

pub enum RecipeRequirement {
    Material {
        material: Material,
    },
    Shape {
        shape: MaterialShape,
    },
    //Couples two requirements together into one requirement, ex:
    //-Needs to have a pointy shape, and be made of metal - Therefore is spear head
    And {
        a: &'static RecipeRequirement,
        b: &'static RecipeRequirement,
    },
    //Allows for multiple requirements to be grouped into one recipe requirement, ex:
    //-Needs to have the shape of a rock, or a brick, or a hammer head, and be made of stone or metal, then it can be a hammer head
    Or {
        a: &'static RecipeRequirement,
        b: &'static RecipeRequirement,
    },
}

impl RecipeRequirement {
    pub fn requirement_fulfilled(&self, item: Entity, data: &CraftingData) -> bool {
        match self {
            Self::Material { material } => {
                if let Some(mat) = data.material.get(item) {
                    return mat.material == *material;
                }
                false
            }
            Self::Shape { shape } => {
                if let Some(mat) = data.material.get(item) {
                    return mat.shape == *shape;
                }
                false
            }
            Self::And { a, b } => {
                a.requirement_fulfilled(item, data) && b.requirement_fulfilled(item, data)
            }
            Self::Or { a, b } => {
                a.requirement_fulfilled(item, data) || b.requirement_fulfilled(item, data)
            }
        }
    }
}

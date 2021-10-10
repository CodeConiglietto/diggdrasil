use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub enum Diet {
    Carnivorous,
    Herbivorous,
    Omnivorous,
}

impl Diet {
    pub fn can_consume(&self, nutrient_type: &NutrientType) -> bool {
        match self {
            Self::Carnivorous => match nutrient_type {
                NutrientType::Plant | NutrientType::Fungi => false,
                NutrientType::Meat | NutrientType::AnimalProduct => true,
            },
            Self::Herbivorous => match nutrient_type {
                NutrientType::Plant | NutrientType::Fungi => true,
                NutrientType::Meat | NutrientType::AnimalProduct => false,
            },
            Self::Omnivorous => {
                Self::Herbivorous.can_consume(nutrient_type)
                    || Self::Carnivorous.can_consume(nutrient_type)
            }
        }
    }
}

use crate::prelude::*;
use specs::prelude::*;
use specs::World as ECSWorld;
use strum::EnumIter;

#[derive(Clone, Copy, EnumIter)]
pub enum Recipe {
    CampFire,
    Spear,
    Pick,
    Axe,
}

impl Recipe {
    pub fn get_resulting_object_name(&self) -> &'static str {
        match self {
            Self::CampFire => "camp fire",
            Self::Spear => "spear",
            Self::Pick => "pick",
            Self::Axe => "axe",
        }
    }

    pub fn get_ingredient_requirements(&self) -> &'static [RecipeIngredient] {
        match self {
            Self::CampFire => &[
                RecipeIngredient {
                    part_name: "fuel",
                    requirement: RecipeRequirement::And {
                        a: &RecipeRequirement::Material {
                            material: Material::Wood,
                        },
                        b: &RecipeRequirement::Shape {
                            shape: MaterialShape::Log,
                        },
                    },
                },
                RecipeIngredient {
                    part_name: "ring",
                    requirement: RecipeRequirement::And {
                        a: &RecipeRequirement::Material {
                            material: Material::Stone,
                        },
                        b: &RecipeRequirement::Or {
                            a: &RecipeRequirement::Shape {
                                shape: MaterialShape::Rock,
                            },
                            b: &RecipeRequirement::Shape {
                                shape: MaterialShape::Brick,
                            },
                        },
                    },
                },
            ],
            Self::Spear => &[
                RecipeIngredient {
                    part_name: "head",
                    requirement: RecipeRequirement::And {
                        a: &RecipeRequirement::Material {
                            material: Material::Stone,
                        },
                        b: &RecipeRequirement::Shape {
                            shape: MaterialShape::Rock,
                        },
                    },
                },
                RecipeIngredient {
                    part_name: "handle",
                    requirement: RecipeRequirement::And {
                        a: &RecipeRequirement::Material {
                            material: Material::Wood,
                        },
                        b: &RecipeRequirement::Shape {
                            shape: MaterialShape::Stick,
                        },
                    },
                },
            ],
            Self::Pick => &[
                RecipeIngredient {
                    part_name: "head",
                    requirement: RecipeRequirement::And {
                        a: &RecipeRequirement::Material {
                            material: Material::Stone,
                        },
                        b: &RecipeRequirement::Shape {
                            shape: MaterialShape::Rock,
                        },
                    },
                },
                RecipeIngredient {
                    part_name: "handle",
                    requirement: RecipeRequirement::And {
                        a: &RecipeRequirement::Material {
                            material: Material::Wood,
                        },
                        b: &RecipeRequirement::Shape {
                            shape: MaterialShape::Stick,
                        },
                    },
                },
            ],
            Self::Axe => &[
                RecipeIngredient {
                    part_name: "head",
                    requirement: RecipeRequirement::And {
                        a: &RecipeRequirement::Material {
                            material: Material::Stone,
                        },
                        b: &RecipeRequirement::Shape {
                            shape: MaterialShape::Rock,
                        },
                    },
                },
                RecipeIngredient {
                    part_name: "handle",
                    requirement: RecipeRequirement::And {
                        a: &RecipeRequirement::Material {
                            material: Material::Wood,
                        },
                        b: &RecipeRequirement::Shape {
                            shape: MaterialShape::Stick,
                        },
                    },
                },
            ],
        }
    }

    pub fn fulfillable_with_inventory_contents(
        &self,
        inv: &InventoryComponent,
        crafting_data: &CraftingData,
    ) -> bool {
        self.get_ingredient_requirements().iter().all(|ingredient| {
            inv.items
                .iter()
                .filter_map(|slot| slot.as_ref())
                .any(|item| {
                    ingredient
                        .requirement
                        .requirement_fulfilled(*item, crafting_data)
                })
        })
    }

    pub fn craft(
        &self,
        ecs_world: &mut ECSWorld,
        _entity_map: EntityMapResource,
        ingredients: Vec<Entity>,
        data: &CraftingData,
    ) -> Result<Entity, String> {
        let requirements = self.get_ingredient_requirements();

        if requirements.len() != ingredients.len() {
            return Err(String::from(
                "Incorrect amount of ingredients passed to crafting recipe!",
            ));
        }

        for (requirement, ingredient) in requirements.iter().zip(ingredients.iter()) {
            if !requirement
                .requirement
                .requirement_fulfilled(*ingredient, data)
            {
                return Err(String::from(
                    "Ingredients passed to crafting recipe do not fulfill requirements",
                ));
            }
        }

        let entity = match self {
            Self::CampFire => FurnitureBuilder::CampFire.build(ecs_world),
            Self::Spear => EquipmentBuilder::Spear{head_material: Material::Stone, handle_material: Material::Wood}.build(ecs_world),
            Self::Pick => EquipmentBuilder::Pick{head_material: Material::Stone, handle_material: Material::Wood}.build(ecs_world),
            Self::Axe => EquipmentBuilder::Axe{head_material: Material::Stone, handle_material: Material::Wood}.build(ecs_world),
            _ => todo!(),
        };

        return Ok(entity);
    }
}

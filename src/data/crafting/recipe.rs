use specs::prelude::*;
use strum::EnumIter;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Recipe {
    CampFire,
    Spear,
    Pick,
    Axe,
    Knife,
}

impl Recipe {
    pub fn get_resulting_object_name(&self) -> &'static str {
        match self {
            Self::CampFire => "camp fire",
            Self::Spear => "spear",
            Self::Pick => "pick",
            Self::Axe => "axe",
            Self::Knife => "knife",
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
            Self::Knife => &[
                RecipeIngredient {
                    part_name: "blade",
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
        ingredients: &[Entity],
        lazy: &LazyUpdate,
        entities: &Entities,
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
            Self::CampFire => FurnitureBuilder::CampFire.build(lazy, entities),
            Self::Spear => EquipmentBuilder::Spear {
                head_material: Material::Stone,
                handle_material: Material::Wood,
            }
            .build(lazy, entities),
            Self::Pick => EquipmentBuilder::Pick {
                head_material: Material::Stone,
                handle_material: Material::Wood,
            }
            .build(lazy, entities),
            Self::Axe => EquipmentBuilder::Axe {
                head_material: Material::Stone,
                handle_material: Material::Wood,
            }
            .build(lazy, entities),
            Self::Knife => EquipmentBuilder::Knife {
                blade_material: Material::Stone,
                handle_material: Material::Wood,
            }
            .build(lazy, entities),
        };

        return Ok(entity);
    }
}

use crate::prelude::*;
use specs::prelude::*;

#[derive(Clone)]
pub enum AIGoal {
    // Wander,
    MoveInDirection {
        x: i32,
        y: i32,
    },
    PickUpItem {
        item: Entity,
    },
    DropItem {
        item: Entity,
    },
    EatItem {
        item: Option<Entity>,
    },
    Build {
        x: i32,
        y: i32,
        tile_type: Option<TileType>,
        consumed_entity: Option<Entity>,
    },
    Craft {
        recipe: Option<Recipe>,
        ingredients: Vec<Entity>,
    },
    // MoveToTile{x: i32, y: i32},
    // KillEntity { target: Entity },
    // AttackEntity { target: Entity },
    // AcquireFood,
    // FleeDanger
}

impl AIGoal {
    pub fn get_textual_representation(&self, data: &RenderData) -> String {
        match self {
            Self::MoveInDirection { x, y } => {
                format!("Move towards {}, {}", x, y)
            }
            Self::PickUpItem { item } => {
                format!("Pick up {}", data.name.get(*item).unwrap().name)
            }
            Self::DropItem { item } => {
                format!("Drop {}", data.name.get(*item).unwrap().name)
            }
            Self::EatItem { item } => {
                let consumed_entity_name =
                    if let Some(name_component) = item.map(|e| data.name.get(e).unwrap()) {
                        &name_component.name
                    } else {
                        "something"
                    };

                format!("Eat {}", consumed_entity_name)
            }
            Self::Build {
                x,
                y,
                tile_type,
                consumed_entity,
            } => {
                let tile_name = if let Some(tile_type_known) = tile_type {
                    tile_type_known.get_name()
                } else {
                    String::from("something")
                };

                let consumed_entity_name = if let Some(name_component) =
                    consumed_entity.map(|e| data.name.get(e).unwrap())
                {
                    &name_component.name
                } else {
                    "something"
                };

                format!(
                    "Build {} at ({}, {}) from {}",
                    tile_name, x, y, consumed_entity_name
                )
            }
            Self::Craft {
                recipe,
                ingredients,
            } => {
                let recipe_name = if let Some(recipe) = recipe {
                    recipe.get_resulting_object_name()
                } else {
                    "something"
                };

                //TODO: have the string print the ingredients if they exist

                format!("Craft a {}", recipe_name)
            }
        }
    }
}

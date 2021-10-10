use crate::prelude::*;
use specs::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AIGoal {
    Wander,
    MoveInDirection {
        direction: Direction,
    },
    AttackInDirection {
        direction: Direction,
    },
    TravelPath {
        path: Vec<(i32, i32)>,
    },
    TravelToPosition {
        target_pos: (i32, i32),
    },
    StowItem {
        item: Entity,
    },
    DropItem {
        item: Entity,
    },
    HoldItem {
        item: Option<Entity>,
    },
    Eat {
        target: Option<Entity>,
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
    FulfilHunger,
    FleeDanger,
    GroupWithAllies,
    KillEntity { target: Entity },
    AttackEntity { target: Entity },
}

impl AIGoal {
    pub fn get_textual_representation(&self, data: &RenderData) -> String {
        match self {
            Self::Wander => String::from("Wander"),
            Self::MoveInDirection { direction } => {
                format!("Move towards {:?}", direction)
            }
            Self::AttackInDirection { direction } => {
                format!("Attack towards {:?}", direction)
            }
            Self::TravelPath { path } => {
                if let Some(dest) = path.first() {
                    format!("Travel to {:?}", dest)
                } else {
                    String::from("Travel somewhere")
                }
            }
            Self::TravelToPosition { target_pos } => {
                format!("Travel to {:?}", target_pos)
            }
            Self::StowItem { item } => {
                format!("Stow {}", data.name.get(*item).unwrap().name)
            }
            Self::DropItem { item } => {
                format!("Drop {}", data.name.get(*item).unwrap().name)
            }
            Self::HoldItem { item } => {
                let consumed_entity_name =
                    if let Some(name_component) = item.map(|e| data.name.get(e).unwrap()) {
                        &name_component.name
                    } else {
                        "something"
                    };

                format!("Hold {}", consumed_entity_name)
            }
            Self::Eat { target } => {
                let consumed_entity_name =
                    if let Some(name_component) = target.map(|e| data.name.get(e).unwrap()) {
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
            Self::Craft { recipe, .. } => {
                let recipe_name = if let Some(recipe) = recipe {
                    recipe.get_resulting_object_name()
                } else {
                    "something"
                };
 
                //TODO: have the string print the ingredients if they exist

                format!("Craft a {}", recipe_name)
            }
            Self::FulfilHunger => String::from("Fulfil hunger"),
            Self::FleeDanger => String::from("Flee from danger"),
            Self::GroupWithAllies => String::from("Group with similar creatures"),
            Self::AttackEntity{target} => format!("Attack {}", data.name.get(*target).unwrap().name),
            Self::KillEntity{target} => format!("Kill {}", data.name.get(*target).unwrap().name),
        }
    }
}

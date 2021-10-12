use crate::prelude::*;
use specs::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AIGoal {
    Wander(WanderGoal),
    MoveInDirection(MoveInDirectionGoal),
    AttackInDirection(AttackInDirectionGoal),
    TravelPath(TravelPathGoal),
    TravelToPosition(TravelToPositionGoal),
    StowItem(StowItemGoal),
    DropItem(DropItemGoal),
    HoldItem(HoldItemGoal),
    Eat(EatGoal),
    Build(BuildGoal),
    Craft(CraftGoal),
    FulfilHunger(FulfilHungerGoal),
    FleeDanger(FleeDangerGoal),
    GroupWithAllies(GroupWithAlliesGoal),
    KillEntity(KillEntityGoal),
    AttackEntity(AttackEntityGoal),
}

impl AIGoal {
    pub fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        match self {
            Self::Wander(goal) => goal.resolve(parent_entity, data),
            Self::MoveInDirection(goal) => goal.resolve(parent_entity, data),
            Self::AttackInDirection(goal) => goal.resolve(parent_entity, data),
            Self::TravelPath(goal) => goal.resolve(parent_entity, data),
            Self::TravelToPosition(goal) => goal.resolve(parent_entity, data),
            Self::StowItem(goal) => goal.resolve(parent_entity, data),
            Self::DropItem(goal) => goal.resolve(parent_entity, data),
            Self::HoldItem(goal) => goal.resolve(parent_entity, data),
            Self::Eat(goal) => goal.resolve(parent_entity, data),
            Self::Build(goal) => goal.resolve(parent_entity, data),
            Self::Craft(goal) => goal.resolve(parent_entity, data),
            Self::FulfilHunger(goal) => goal.resolve(parent_entity, data),
            Self::FleeDanger(goal) => goal.resolve(parent_entity, data),
            Self::GroupWithAllies(goal) => goal.resolve(parent_entity, data),
            Self::KillEntity(goal) => goal.resolve(parent_entity, data),
            Self::AttackEntity(goal) => goal.resolve(parent_entity, data),
        }
    }

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
                pos,
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
                    "Build {} at {} from {}",
                    tile_name, pos, consumed_entity_name
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
            Self::AttackEntity { target } => {
                format!("Attack {}", data.name.get(*target).unwrap().name)
            }
            Self::KillEntity { target } => format!("Kill {}", data.name.get(*target).unwrap().name),
        }
    }
}

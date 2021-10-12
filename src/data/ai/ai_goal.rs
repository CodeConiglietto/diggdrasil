use crate::prelude::*;
use specs::prelude::*;

#[derive(Debug, Clone)]
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

impl AIGoalTrait for AIGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
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

    fn get_textual_representation(&self, data: &RenderData) -> String {
        match self {
            Self::Wander(goal) => goal.get_textual_representation(data),
            Self::MoveInDirection(goal) => goal.get_textual_representation(data),
            Self::AttackInDirection(goal) => goal.get_textual_representation(data),
            Self::TravelPath(goal) => goal.get_textual_representation(data),
            Self::TravelToPosition(goal) => goal.get_textual_representation(data),
            Self::StowItem(goal) => goal.get_textual_representation(data),
            Self::DropItem(goal) => goal.get_textual_representation(data),
            Self::HoldItem(goal) => goal.get_textual_representation(data),
            Self::Eat(goal) => goal.get_textual_representation(data),
            Self::Build(goal) => goal.get_textual_representation(data),
            Self::Craft(goal) => goal.get_textual_representation(data),
            Self::FulfilHunger(goal) => goal.get_textual_representation(data),
            Self::FleeDanger(goal) => goal.get_textual_representation(data),
            Self::GroupWithAllies(goal) => goal.get_textual_representation(data),
            Self::KillEntity(goal) => goal.get_textual_representation(data),
            Self::AttackEntity(goal) => goal.get_textual_representation(data),
        }
    }
}

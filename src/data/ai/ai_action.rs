use specs::Entity;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum AIAction {
    MoveInDirection {
        x: i32,
        y: i32,
    },
    AttackInDirection {
        direction: Direction,
        attack: Attack,
        attack_offsets: Option<Vec<(i32, i32)>>,
    },
    AttackEntity {
        target: Entity,
    },
    StowItemFromGround {
        item: Entity,
    },
    StowHeldItem,
    DropItemFromInventory {
        item: Entity,
    },
    HoldItemFromInventory {
        item: Entity,
    },
    EatItemFromInventory {
        item: Entity,
    },
    BuildAtLocation {
        x: i32,
        y: i32,
        tile_type: TileType,
        consumed_entity: Entity,
    },
    Craft {
        recipe: Recipe,
        ingredients: Vec<Entity>,
    },
}

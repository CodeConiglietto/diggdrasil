use specs::Entity;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum AIAction {
    MoveInDirection {
        offset: IPosition,
    },
    AttackInDirection {
        direction: Direction,
        attack: Attack,
        attack_offsets: Option<Vec<IPosition>>,
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
    EatFromGround {
        target: Entity,
    },
    BuildAtLocation {
        pos: IPosition,
        tile_type: TileType,
        consumed_entity: Entity,
    },
    Craft {
        recipe: Recipe,
        ingredients: Vec<Entity>,
    },
}

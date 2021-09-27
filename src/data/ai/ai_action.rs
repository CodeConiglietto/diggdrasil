use specs::Entity;

use crate::prelude::*;

#[derive(Clone)]
pub enum AIAction {
    MoveInDirection {
        x: i32,
        y: i32,
    },
    AttackEntity {
        target: Entity,
    },
    PickUpItem {
        item: Entity,
    },
    DropItem {
        item: Entity,
    },
    EatItem {
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

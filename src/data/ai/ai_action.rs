use specs::Entity;

pub enum AIAction {
    MoveInDirection { x: i32, y: i32 },
    AttackEntity { target: Entity },
    PickUpItem { item: Entity },
    DropItem { item: Entity },
}

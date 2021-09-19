use specs::{Component, Entity, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct InventoryComponent {
    pub items: [Option<Entity>; 10],
}

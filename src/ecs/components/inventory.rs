use specs::{Component, Entity, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct InventoryComponent {
    pub items: [Option<Entity>; 10],
}

impl InventoryComponent {
    pub fn insert(&mut self, item: Entity) -> bool {
        for i in 0..self.items.len() {
            if self.items[i].is_none() {
                self.items[i] = Some(item);

                return true
            }
        }

        false
    }
}

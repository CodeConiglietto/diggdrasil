use specs::{Component, Entity, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct ManipulatorComponent {
    pub held_item: Option<Entity>,
}

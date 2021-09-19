use specs::{Component, Entity, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct ManipulationComponent {
    pub held_item: Option<Entity>,
}

use specs::{Component, Entity, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct DeathComponent {
    pub contained_entities: Vec<Entity>,
}

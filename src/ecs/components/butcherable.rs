use specs::{Component, Entity, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct ButcherableComponent {
    pub yielded_entities: Vec<Entity>,
}

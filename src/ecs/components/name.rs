use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct NameComponent {
    pub name: String,
}

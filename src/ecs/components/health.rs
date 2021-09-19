use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct HealthComponent {
    pub value: u32,
    pub max_value: u32,
}

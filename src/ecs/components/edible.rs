use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct EdibleComponent {
    pub nutrient_value: usize,
}

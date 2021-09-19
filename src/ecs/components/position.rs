use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct PositionComponent {
    pub x: i32,
    pub y: i32,
}

use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct IntendedMovementComponent {
    pub x_delta: i32,
    pub y_delta: i32,
    pub controlled: bool,
}

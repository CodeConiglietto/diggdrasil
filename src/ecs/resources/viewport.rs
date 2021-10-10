use crate::prelude::*;

#[derive(Default)]
pub struct ViewportResource {
    pub camera_world_position: IPosition,
}

impl ViewportResource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_viewport_bounds(&self) -> (i32, i32, i32, i32) {
        let left = self.camera_world_position.x - MAP_X_SIZE as i32 / 2;
        let right = left + MAP_X_SIZE as i32;
        let top = self.camera_world_position.y - MAP_Y_SIZE as i32 / 2;
        let bottom = top + MAP_Y_SIZE as i32;

        (left, right, top, bottom)
    }
}

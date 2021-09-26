use crate::prelude::*;
use specs::prelude::*;

#[derive(SystemData)]
pub struct WorldData<'a> {
    pub position: WriteStorage<'a, PositionComponent>,

    pub lazy: Read<'a, LazyUpdate>,
    pub entities: Entities<'a>,
}

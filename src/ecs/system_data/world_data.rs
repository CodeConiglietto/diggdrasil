use specs::prelude::*;

use crate::prelude::*;

#[derive(SystemData)]
pub struct WorldData<'a> {
    pub position: WriteStorage<'a, PositionComponent>,
    pub id: WriteStorage<'a, IdComponent>,

    pub lazy: Read<'a, LazyUpdate>,
    pub entities: Entities<'a>,
}

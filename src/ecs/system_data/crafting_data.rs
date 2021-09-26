use crate::prelude::*;
use specs::prelude::*;

#[derive(SystemData)]
pub struct CraftingData<'a> {
    //Read components
    pub material: ReadStorage<'a, MaterialComponent>,
}

use specs::prelude::*;

use crate::prelude::*;

#[derive(SystemData)]
pub struct GoalData<'a> {
    //Entities
    pub entities: Entities<'a>,

    //Included system data
    pub crafting_data: CraftingData<'a>,

    //Resources
    pub tile_world: ReadExpect<'a, TileWorldResource>,

    //Read Components
    pub attack: ReadStorage<'a, AttackComponent>,
    pub edible: ReadStorage<'a, EdibleComponent>,
    pub digestion: ReadStorage<'a, DigestionComponent>,
    pub health: ReadStorage<'a, HealthComponent>,
    pub inventory: ReadStorage<'a, InventoryComponent>,
    pub manipulator: ReadStorage<'a, ManipulatorComponent>,
    pub name: ReadStorage<'a, NameComponent>,
    pub perception: ReadStorage<'a, AIPerceptionComponent>,
    pub personality: ReadStorage<'a, AIPersonalityComponent>,
    pub position: ReadStorage<'a, PositionComponent>,

    //Write components
    pub input: WriteStorage<'a, InputComponent>,
    pub pathing: WriteStorage<'a, PathingComponent>,
}
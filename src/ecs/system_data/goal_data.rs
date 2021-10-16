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
    pub collider: ReadStorage<'a, ColliderComponent>,
    pub edible: ReadStorage<'a, EdibleComponent>,
    pub field_of_view: ReadStorage<'a, FieldOfViewComponent>,
    pub digestion: ReadStorage<'a, DigestionComponent>,
    pub health: ReadStorage<'a, HealthComponent>,
    pub input: ReadStorage<'a, InputComponent>,
    pub inventory: ReadStorage<'a, InventoryComponent>,
    pub manipulator: ReadStorage<'a, ManipulatorComponent>,
    pub name: ReadStorage<'a, NameComponent>,
    pub perception: ReadStorage<'a, AIPerceptionComponent>,
    pub personality: ReadStorage<'a, AIPersonalityComponent>,
    pub position: ReadStorage<'a, PositionComponent>,

    //Write components
    pub pathing: WriteStorage<'a, PathingComponent>,
}

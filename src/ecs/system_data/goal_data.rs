use crate::prelude::*;

pub struct GoalData<'a> {
    //Entities
    entities:    Entities<'a>,
    
    //Included system data
    crafting_data:    CraftingData<'a>,

    //Resources
    twld:    ReadExpect<'a, TileWorldResource>,

    //Read Components
    attack:    ReadStorage<'a, AttackComponent>,
    edible:    ReadStorage<'a, EdibleComponent>,
    digestion:    ReadStorage<'a, DigestionComponent>,
    position:    ReadStorage<'a, PositionComponent>,
    health:    ReadStorage<'a, HealthComponent>,
    inventory:    ReadStorage<'a, InventoryComponent>,
    manipulator:    ReadStorage<'a, ManipulatorComponent>,
    name:    ReadStorage<'a, NameComponent>,
    perception:    ReadStorage<'a, AIPerceptionComponent>,
    personality:    ReadStorage<'a, AIPersonalityComponent>,

    //Write components
    goal:    WriteStorage<'a, AIGoalComponent>,
    action:    WriteStorage<'a, AIActionComponent>,
    input:    WriteStorage<'a, InputComponent>,
    pathing:    WriteStorage<'a, PathingComponent>,
}
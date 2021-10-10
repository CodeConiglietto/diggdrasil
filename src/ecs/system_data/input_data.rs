use crate::prelude::*;
use specs::prelude::*;

#[derive(SystemData)]
pub struct InputData<'a> {
    //Entities
    pub entities: Entities<'a>,
    pub lazy: Read<'a, LazyUpdate>,

    //Resources
    pub keyboard: ReadExpect<'a, KeyboardResource>,
    pub mouse: ReadExpect<'a, MouseResource>,
    pub tile_world: ReadExpect<'a, TileWorldResource>,

    //Read components
    pub butcherable: ReadStorage<'a, ButcherableComponent>,
    pub collider: ReadStorage<'a, ColliderComponent>,
    pub collision: ReadStorage<'a, CollisionComponent>,
    pub death: ReadStorage<'a, DeathComponent>,
    pub draw: ReadStorage<'a, DrawComponent>,
    pub health: ReadStorage<'a, HealthComponent>,
    pub intended_movement: ReadStorage<'a, IntendedMovementComponent>,
    pub item: ReadStorage<'a, ItemComponent>,
    pub manipulation: ReadStorage<'a, ManipulatorComponent>,
    pub name: ReadStorage<'a, NameComponent>,
    pub particle: ReadStorage<'a, ParticleComponent>,
    pub position: ReadStorage<'a, PositionComponent>,
    pub velocity: ReadStorage<'a, VelocityComponent>,

    //Write components
    pub ai_goal: WriteStorage<'a, AIGoalComponent>,
    pub input: WriteStorage<'a, InputComponent>,
    pub inventory: WriteStorage<'a, InventoryComponent>,
    pub pathing: WriteStorage<'a, PathingComponent>,
}

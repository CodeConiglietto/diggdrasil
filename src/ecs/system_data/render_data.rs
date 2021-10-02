use crate::prelude::*;
use specs::prelude::*;

#[derive(SystemData)]
pub struct RenderData<'a> {
    //Resources
    pub keyboard: ReadExpect<'a, KeyboardResource>,
    pub tile_world: ReadExpect<'a, TileWorldResource>,
    pub particle_map: ReadExpect<'a, ParticleMapResource>,

    //Read components
    pub butcherable: ReadStorage<'a, ButcherableComponent>,
    pub collider: ReadStorage<'a, ColliderComponent>,
    pub collision: ReadStorage<'a, CollisionComponent>,
    pub death: ReadStorage<'a, DeathComponent>,
    pub digestion: ReadStorage<'a, DigestionComponent>,
    pub draw: ReadStorage<'a, DrawComponent>,
    pub edible: ReadStorage<'a, EdibleComponent>,
    pub field_of_view: ReadStorage<'a, FieldOfViewComponent>,
    pub health: ReadStorage<'a, HealthComponent>,
    pub intended_movement: ReadStorage<'a, IntendedMovementComponent>,
    pub inventory: ReadStorage<'a, InventoryComponent>,
    pub item: ReadStorage<'a, ItemComponent>,
    pub manipulator: ReadStorage<'a, ManipulatorComponent>,
    pub name: ReadStorage<'a, NameComponent>,
    pub particle: ReadStorage<'a, ParticleComponent>,
    pub position: ReadStorage<'a, PositionComponent>,
    pub velocity: ReadStorage<'a, VelocityComponent>,
    pub ai_goal: ReadStorage<'a, AIGoalComponent>,
    pub input: ReadStorage<'a, InputComponent>,
}

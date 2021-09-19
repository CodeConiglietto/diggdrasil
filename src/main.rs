use bunnyfont::ggez::{GgBunnyFont, GgBunnyFontBatch};
use ggez::{
    event::{self, KeyCode},
    graphics::{self, DrawParam, FilterMode, Image},
    input::keyboard,
    Context, GameResult,
};
use glam::*;
use ndarray::prelude::*;
use rand::prelude::*;
use specs::{Builder, Join, RunNow, World as ECSWorld, WorldExt as ECSWorldExt};
use std::{env, path::PathBuf};
use tui::{
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

use crate::prelude::*;

pub mod prelude;

pub mod constants;
pub mod data;
pub mod ecs;
pub mod util;

struct MainState {
    //Assets
    font_batch: GgBunnyFontBatch,
    tui: Terminal<Ui>,

    //World architecture
    ecs_world: ECSWorld,

    //Systems in the order which they are run
    input_resolution_system: InputResolutionSystem,
    goal_resolution_system: GoalResolutionSystem,
    action_resolution_system: ActionResolutionSystem,
    collision_calculation_system: CollisionCalculationSystem,
    movement_resolution_system: MovementResolutionSystem,
    collision_resolution_system: CollisionResolutionSystem,
    health_resolution_system: HealthResolutionSystem,
    particle_system: ParticleSystem,

    //Player and UI variables
    symbolic_view: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut texture = Image::new(ctx, "/master8x8.png")?;
        texture.set_filter(FilterMode::Nearest);

        //Register all components
        let mut ecs_world = ECSWorld::new();
        ecs_world.register::<AIActionComponent>();
        ecs_world.register::<AIGoalComponent>();
        ecs_world.register::<ButcherableComponent>();
        ecs_world.register::<ColliderComponent>();
        ecs_world.register::<CollisionComponent>();
        ecs_world.register::<DeathComponent>();
        ecs_world.register::<DrawComponent>();
        ecs_world.register::<HealthComponent>();
        ecs_world.register::<InputComponent>();
        ecs_world.register::<IntendedMovementComponent>();
        ecs_world.register::<InventoryComponent>();
        ecs_world.register::<ItemComponent>();
        ecs_world.register::<ManipulatorComponent>();
        ecs_world.register::<NameComponent>();
        ecs_world.register::<ParticleComponent>();
        ecs_world.register::<PositionComponent>();
        ecs_world.register::<VelocityComponent>();

        //Initialise all resources
        let keyboard = KeyboardResource {
            last_pressed_key: None,
        };

        let tile_map = TileMapResource {
            contents: Array2::from_shape_fn((MAP_X_SIZE, MAP_Y_SIZE), |(_x, _y)| Tile {
                seed: thread_rng().gen::<usize>(),
                tile_type: if thread_rng().gen_range(0.0..1.0) > 0.75 {
                    TileType::Wall
                } else {
                    TileType::Ground
                },
            }),
        };

        let mut entity_map = EntityMapResource {
            contents: Array2::from_shape_fn((MAP_X_SIZE, MAP_Y_SIZE), |(_x, _y)| Vec::new()),
        };

        let particle_map = ParticleMapResource::default();

        // TODO
        //let ui = UiResource { terminal: };

        //Insert pertinent data into resources
        let player = CreatureBuilder::Humanoid { race: Race::Human }.build(&mut ecs_world, true);
        entity_map.spawn_entity(player, (16, 16), &mut ecs_world.system_data());

        for _ in 0..16 {
            let tree = CreatureBuilder::Tree.build(&mut ecs_world, true);
            entity_map.spawn_entity(
                tree,
                (thread_rng().gen_range(0..32), thread_rng().gen_range(0..32)),
                &mut ecs_world.system_data(),
            );
        }

        //Assign resources to ecs world
        ecs_world.insert(keyboard);
        ecs_world.insert(tile_map);
        ecs_world.insert(entity_map);
        ecs_world.insert(particle_map);
        // TODO
        //ecs_world.insert(ui);

        //Construct game state
        let s = MainState {
            //Assets
            font_batch: GgBunnyFontBatch::new(GgBunnyFont::new(texture.clone(), (8, 8))).unwrap(),
            tui: Terminal::new(Ui::new(
                GgBunnyFontBatch::new(GgBunnyFont::new(texture, (8, 8))).unwrap(),
                (MAP_X_SIZE + INVENTORY_SIZE, MAP_Y_SIZE),
                RENDER_SCALE,
            ))
            .unwrap(),

            //World architecture
            ecs_world,

            //Systems in the order which they are run
            input_resolution_system: InputResolutionSystem,
            goal_resolution_system: GoalResolutionSystem,
            action_resolution_system: ActionResolutionSystem,
            collision_calculation_system: CollisionCalculationSystem,
            movement_resolution_system: MovementResolutionSystem,
            collision_resolution_system: CollisionResolutionSystem,
            health_resolution_system: HealthResolutionSystem,
            particle_system: ParticleSystem,

            //Player and UI variables
            symbolic_view: false,
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //Process input
        if keyboard::is_key_pressed(ctx, KeyCode::LAlt) {
            self.symbolic_view = true;
        } else {
            self.symbolic_view = false;
        }

        self.ecs_world
            .create_entity()
            .with(ParticleComponent {
                position: (
                    thread_rng().gen_range(0..MAP_X_SIZE) as i32,
                    thread_rng().gen_range(0..MAP_Y_SIZE) as i32,
                    thread_rng().gen_range(0..MAX_PARTICLE_HEIGHT),
                ),
                particle_type: ParticleType::Rain {
                    initial_angle: Direction::Left,
                },
            })
            .build();

        //Write resources
        let last_keypress = self
            .ecs_world
            .read_resource::<KeyboardResource>()
            .last_pressed_key;
        let next_keypress = keyboard::pressed_keys(ctx).iter().next().copied();
        let mut final_keypress = None;

        //TODO: find proper system to prevent or slow keypresses
        if last_keypress != next_keypress {
            final_keypress = next_keypress;
        }

        self.ecs_world
            .write_resource::<KeyboardResource>()
            .last_pressed_key = final_keypress;

        self.ecs_world
            .write_resource::<ParticleMapResource>()
            .clear_all();

        //Run systems in order
        self.input_resolution_system.run_now(&self.ecs_world);
        self.goal_resolution_system.run_now(&self.ecs_world);
        self.action_resolution_system.run_now(&self.ecs_world);
        self.collision_calculation_system.run_now(&self.ecs_world);
        self.movement_resolution_system.run_now(&self.ecs_world);
        self.collision_resolution_system.run_now(&self.ecs_world);
        self.health_resolution_system.run_now(&self.ecs_world);
        self.particle_system.run_now(&self.ecs_world);

        self.ecs_world.maintain();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.05, 0.05, 0.05, 1.0].into());

        self.font_batch.clear();

        let data: RenderData = self.ecs_world.system_data();

        let tiles = &data.tile_map.contents;

        for y in 0..MAP_Y_SIZE {
            for x in 0..MAP_X_SIZE {
                let ix = x as i32;
                let iy = y as i32;

                let tile = &tiles[[x, y]];

                if self.symbolic_view {
                    tile.get_symbolbuilder().get_symbol().draw_to_font_batch(
                        &mut self.font_batch,
                        (ix, iy),
                        RENDER_SCALE,
                    );
                } else {
                    tile.get_spritebuilder().get_sprite().draw_to_font_batch(
                        &mut self.font_batch,
                        (ix, iy),
                        RENDER_SCALE,
                    );
                }

                for entity in data.entity_map.contents[[x, y]].iter() {
                    let dc = data.draw.get(*entity).unwrap();

                    if self.symbolic_view {
                        if let Some(sym_build) = &dc.symbol_builder {
                            sym_build.get_symbol().draw_to_font_batch(
                                &mut self.font_batch,
                                (ix, iy),
                                RENDER_SCALE,
                            );
                        }
                    } else {
                        dc.sprite_builder.get_sprite().draw_to_font_batch(
                            &mut self.font_batch,
                            (ix, iy),
                            RENDER_SCALE,
                        );
                    }
                }
            }

            for particle in data.particle_map.contents[y].iter() {
                let pac = data.particle.get(*particle).unwrap();
                let (x, y, z) = pac.position;
                let (ix, iy) = (x, y - z);

                pac.particle_type.get_char().draw_to_font_batch(
                    &mut self.font_batch,
                    (ix, iy),
                    RENDER_SCALE,
                );
            }
        }

        self.tui
            .draw(|f| {
                for (inventory, _input) in (&data.inventory, &data.input).join() {
                    let list = List::new(
                        inventory
                            .items
                            .iter()
                            .enumerate()
                            .map(|(i, slot)| {
                                if let Some(item) = slot {
                                    let c = char::from(u32::from('a') as u8 + i as u8); // TODO Change this to a sane function later

                                    let name = data.name.get(*item).unwrap();
                                    ListItem::new(format!("{}) {}", c, name.name))
                                } else {
                                    ListItem::new("")
                                }
                            })
                            .collect::<Vec<_>>(),
                    );

                    let block = Block::default().title("Inventory").borders(Borders::ALL);

                    f.render_widget(
                        list.block(block),
                        tui::layout::Rect::new(MAP_X_SIZE as u16, 0, INVENTORY_SIZE as u16, 12),
                    );
                }

                for input in (&data.input).join() {
                    if let Some(popup) = &input.popup {
                        popup.render(
                            f,
                            tui::layout::Rect::new(0, 0, MAP_X_SIZE as u16, MAP_Y_SIZE as u16),
                            &data,
                        );
                    }
                }
            })
            .unwrap();

        ggez::graphics::draw(ctx, &mut self.font_batch, DrawParam::default())?;
        ggez::graphics::draw(ctx, self.tui.backend_mut(), DrawParam::default())?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.3f]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .level_for("gfx_device_gl", log::LevelFilter::Warn)
        .level_for("winit", log::LevelFilter::Info)
        .level_for("gilrs", log::LevelFilter::Warn)
        .level_for("ggez", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let mut cb = ggez::ContextBuilder::new("Diggdrasil", "CodeBunny");

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        // info!("Adding cargo resource path: '{:?}'", path);
        cb = cb.add_resource_path(path);
    }

    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

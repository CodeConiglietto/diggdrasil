use bunnyfont::ggez::{GgBunnyFont, GgBunnyFontBatch};
use ggez::{
    conf::WindowMode,
    event::{self, KeyCode, KeyMods},
    graphics::{self, DrawParam, FilterMode, Image},
    input::keyboard,
    Context, GameResult,
};
use glam::*;
use itertools::Itertools;
use ndarray::prelude::*;
use rand::prelude::*;
use specs::{Builder, Join, RunNow, World as ECSWorld, WorldExt as ECSWorldExt};
use std::{env, path::PathBuf, time::Duration};
use tui::{
    layout::{Constraint, Direction as LayoutDirection, Layout},
    style::{Color as TuiColor, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Terminal,
};

use crate::prelude::*;

pub mod prelude;

pub mod constants;
pub mod data;
pub mod ecs;
pub mod generation;
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
    digestion_resolution_system: DigestionResolutionSystem,
    health_resolution_system: HealthResolutionSystem,
    particle_system: ParticleSystem,

    //Player and UI variables
    symbolic_view: bool,

    current_tic: u32,
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
        ecs_world.register::<DigestionComponent>();
        ecs_world.register::<DrawComponent>();
        ecs_world.register::<EdibleComponent>();
        ecs_world.register::<HealthComponent>();
        ecs_world.register::<InputComponent>();
        ecs_world.register::<IntendedMovementComponent>();
        ecs_world.register::<InventoryComponent>();
        ecs_world.register::<ItemComponent>();
        ecs_world.register::<ManipulatorComponent>();
        ecs_world.register::<MaterialComponent>();
        ecs_world.register::<NameComponent>();
        ecs_world.register::<ParticleComponent>();
        ecs_world.register::<PositionComponent>();
        ecs_world.register::<VelocityComponent>();

        //Initialise all resources
        let keyboard = KeyboardResource {
            last_pressed_key: None,
            modifiers: KeyMods::default(),
        };

        let mut tile_map = TileMapResource {
            contents: Array2::from_shape_fn((MAP_X_SIZE, MAP_Y_SIZE), |(_x, _y)| Tile {
                seed: thread_rng().gen::<usize>(),
                tile_type: if thread_rng().gen_range(0.0..1.0) > 0.75 {
                    if thread_rng().gen_range(0.0..1.0) > 0.5 {
                        TileType::Wall {
                            material: Material::Stone,
                        }
                    } else {
                        TileType::ConstructedWall {
                            material: Material::Wood,
                            material_shape: MaterialShape::Plank,
                            wall_feature: None,
                        }
                    }
                } else {
                    TileType::Ground
                },
                tile_variant: TileVariant::default(),
            }),
        };

        for x in 12..=20 {
            for y in 12..=20 {
                tile_map.contents[[x, y]] = Tile {
                    seed: thread_rng().gen::<usize>(),
                    tile_type: if y != 16 && (x == 12 || x == 20 || y == 12 || y == 20) {
                        TileType::ConstructedWall {
                            material: Material::Wood,
                            material_shape: MaterialShape::Plank,
                            wall_feature: None,
                        }
                    } else {
                        TileType::Ground
                    },
                    tile_variant: TileVariant::default(),
                }
            }
        }

        tile_map.refresh_all_tile_variants();

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
            let tree = VegetationBuilder::Tree.build(&mut ecs_world);
            entity_map.spawn_entity(
                tree,
                (thread_rng().gen_range(0..32), thread_rng().gen_range(0..32)),
                &mut ecs_world.system_data(),
            );
            let bush = VegetationBuilder::BerryBush.build(&mut ecs_world);
            entity_map.spawn_entity(
                bush,
                (thread_rng().gen_range(0..32), thread_rng().gen_range(0..32)),
                &mut ecs_world.system_data(),
            );
            let stick = ItemBuilder::Stick.build(&mut ecs_world);
            entity_map.spawn_entity(
                stick,
                (thread_rng().gen_range(0..32), thread_rng().gen_range(0..32)),
                &mut ecs_world.system_data(),
            );
            let log = ItemBuilder::Log.build(&mut ecs_world);
            entity_map.spawn_entity(
                log,
                (thread_rng().gen_range(0..32), thread_rng().gen_range(0..32)),
                &mut ecs_world.system_data(),
            );
            let stone = ItemBuilder::Stone.build(&mut ecs_world);
            entity_map.spawn_entity(
                stone,
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

        let (char_width, char_height) = (8, 8);
        let (ui_width, ui_height) = (
            (WINDOW_WIDTH as f32 / (char_width as f32 * RENDER_SCALE)).floor() as usize,
            (WINDOW_HEIGHT as f32 / (char_height as f32 * RENDER_SCALE)).floor() as usize,
        );

        //Construct game state
        let s = MainState {
            //Assets
            font_batch: GgBunnyFontBatch::new(GgBunnyFont::new(
                texture.clone(),
                (char_width, char_height),
            ))
            .unwrap(),
            tui: Terminal::new(Ui::new(
                GgBunnyFontBatch::new(GgBunnyFont::new(texture, (char_width, char_height)))
                    .unwrap(),
                (ui_width, ui_height),
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
            digestion_resolution_system: DigestionResolutionSystem,
            health_resolution_system: HealthResolutionSystem,
            particle_system: ParticleSystem,

            //Player and UI variables
            symbolic_view: false,

            current_tic: 0,
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }

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

        // //TODO: find proper system to prevent or slow keypresses
        // let last_keypress = self
        //     .ecs_world
        //     .read_resource::<KeyboardResource>()
        //     .last_pressed_key;
        // let next_keypress = keyboard::pressed_keys(ctx).iter().next().copied();
        // let mut final_keypress = None;
        // if last_keypress != next_keypress {
        //     final_keypress = next_keypress;
        // }
        // self.ecs_world
        //     .write_resource::<KeyboardResource>()
        //     .last_pressed_key = final_keypress

        self.ecs_world
            .write_resource::<KeyboardResource>()
            .last_pressed_key = keyboard::pressed_keys(ctx).iter().next().copied();

        self.ecs_world
            .write_resource::<KeyboardResource>()
            .modifiers = keyboard::active_mods(ctx);

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
        self.digestion_resolution_system.run_now(&self.ecs_world);
        self.health_resolution_system.run_now(&self.ecs_world);
        self.particle_system.run_now(&self.ecs_world);

        self.ecs_world.maintain();

        self.current_tic += 1;

        std::thread::sleep(Duration::from_millis(50));

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
                    tile.get_symbolbuilder()
                        .get_symbol(tile.seed)
                        .draw_to_font_batch(&mut self.font_batch, (ix, iy), RENDER_SCALE);
                } else {
                    tile.get_spritebuilder()
                        .get_sprite(tile.seed)
                        .draw_to_font_batch(&mut self.font_batch, (ix, iy), RENDER_SCALE);
                }

                for entity in data.entity_map.contents[[x, y]].iter() {
                    let dc = data.draw.get(*entity).unwrap();

                    if self.symbolic_view {
                        if let Some(sym_build) = &dc.symbol_builder {
                            sym_build
                                .get_symbol(entity.id() as usize)
                                .draw_to_font_batch(&mut self.font_batch, (ix, iy), RENDER_SCALE);
                        }
                    } else {
                        dc.sprite_builder
                            .get_sprite(entity.id() as usize)
                            .draw_to_font_batch(&mut self.font_batch, (ix, iy), RENDER_SCALE);
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
                let (left_pane, mut right_pane) = Layout::default()
                    .direction(LayoutDirection::Horizontal)
                    .constraints([Constraint::Length(MAP_X_SIZE as u16), Constraint::Min(0)])
                    .split(f.size())
                    .into_iter()
                    .collect_tuple()
                    .unwrap();
                let (map_pane, bottom_pane) = Layout::default()
                    .direction(LayoutDirection::Vertical)
                    .constraints([
                        Constraint::Length(MAP_Y_SIZE as u16 + 1),
                        Constraint::Min(0),
                    ])
                    .split(left_pane)
                    .into_iter()
                    .collect_tuple()
                    .unwrap();

                if let Some((input, inventory, digestion, health)) = (
                    &data.input,
                    (&data.inventory).maybe(),
                    (&data.digestion).maybe(),
                    (&data.health).maybe(),
                )
                    .join()
                    .next()
                {
                    if let Some(inventory) = inventory {
                        let (inventory_pane, rest) = Layout::default()
                            .direction(LayoutDirection::Vertical)
                            .constraints([
                                Constraint::Length(inventory.items.len() as u16 + 2),
                                Constraint::Min(0),
                            ])
                            .split(right_pane)
                            .into_iter()
                            .collect_tuple()
                            .unwrap();

                        right_pane = rest;

                        let list = List::new(
                            inventory
                                .items
                                .iter()
                                .enumerate()
                                .map(|(i, slot)| {
                                    let c = index_to_letter(i).unwrap();

                                    if let Some(item) = slot {
                                        let name = data.name.get(*item).unwrap();
                                        ListItem::new(format!("{}) {}", c, name.name))
                                    } else {
                                        ListItem::new(format!("{}) -", c))
                                    }
                                })
                                .collect::<Vec<_>>(),
                        );

                        let block = Block::default().title("Inventory").borders(Borders::ALL);

                        f.render_widget(list.block(block), inventory_pane);
                    }

                    if let Some(digestion) = digestion {
                        let (digestion_pane, _rest) = Layout::default()
                            .direction(LayoutDirection::Vertical)
                            .constraints([
                                Constraint::Length(
                                    (digestion.contents.len() as u16).min(10).max(2) + 2,
                                ),
                                Constraint::Min(0),
                            ])
                            .split(right_pane)
                            .into_iter()
                            .collect_tuple()
                            .unwrap();

                        let list = List::new(
                            digestion
                                .contents
                                .iter()
                                .enumerate()
                                .map(|(i, item)| {
                                    let c = index_to_letter(i).unwrap();
                                    let name = data.name.get(*item).unwrap();

                                    ListItem::new(format!("{}) {}", c, name.name))
                                })
                                .collect::<Vec<_>>(),
                        );

                        let block = Block::default()
                            .title(format!(
                                "Stomach ({})",
                                digestion.get_total_nutrition(&data.edible)
                            ))
                            .borders(Borders::ALL);

                        f.render_widget(list.block(block), digestion_pane);
                    }

                    if let Some(health) = health {
                        let max_value_str = format!("{}", health.max_value);

                        let (health_name_rect, health_bar_rect, _) = Layout::default()
                            .direction(LayoutDirection::Vertical)
                            .constraints([
                                Constraint::Length(1),
                                Constraint::Length(1),
                                Constraint::Min(0),
                            ])
                            .split(bottom_pane)
                            .into_iter()
                            .collect_tuple()
                            .unwrap();

                        let (health_gauge_rect, health_display_rect) = Layout::default()
                            .direction(LayoutDirection::Horizontal)
                            .constraints([
                                Constraint::Min(0),
                                Constraint::Length(max_value_str.len() as u16 * 2 + 3),
                            ])
                            .split(health_bar_rect)
                            .into_iter()
                            .collect_tuple()
                            .unwrap();

                        let health_name = Paragraph::new("Health");
                        let health_gauge = Gauge::default()
                            .label("")
                            .ratio(health.value as f64 / health.max_value as f64)
                            .use_unicode(true)
                            .gauge_style(Style::default().fg(TuiColor::Red));
                        let health_display = Paragraph::new(format!(
                            "[{:width$}/{}]",
                            health.value,
                            max_value_str,
                            width = max_value_str.len()
                        ));

                        f.render_widget(health_name, health_name_rect);
                        f.render_widget(health_gauge, health_gauge_rect);
                        f.render_widget(health_display, health_display_rect);
                    }

                    if let Some(popup) = &input.popup {
                        popup.render(f, map_pane, &data);
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

    let mut cb = ggez::ContextBuilder::new("Diggdrasil", "CodeBunny").window_mode(WindowMode {
        width: WINDOW_WIDTH as f32,
        height: WINDOW_HEIGHT as f32,
        ..WindowMode::default()
    });

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

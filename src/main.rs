use std::{convert::TryFrom, fs};

use bunnyfont::ggez::{GgBunnyFont, GgBunnyFontBatch};
use bunnyfont::{
    char_transforms::{CharMirror, CharRotation},
    ggez::GgBunnyChar,
};
use ggez::{
    conf::WindowMode,
    event::{self, KeyCode, KeyMods},
    graphics::{self, Color, DrawParam, FilterMode, Image},
    input::{
        keyboard,
        mouse::{self, MouseButton},
    },
    Context, GameResult,
};
use glam::*;
use itertools::Itertools;
use noise::{Perlin, Seedable};
use rand::prelude::*;
use specs::{
    BitSet, Entities, Join, LazyUpdate, Read, RunNow, World as ECSWorld, WorldExt as ECSWorldExt,
    WriteStorage,
};
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
    weather_system: WeatherSystem,
    input_resolution_system: InputResolutionSystem,
    perception_resolution_system: PerceptionResolutionSystem,
    goal_resolution_system: GoalResolutionSystem,
    action_resolution_system: ActionResolutionSystem,
    collision_calculation_system: CollisionCalculationSystem,
    movement_resolution_system: MovementResolutionSystem,
    collision_resolution_system: CollisionResolutionSystem,
    digestion_resolution_system: DigestionResolutionSystem,
    health_resolution_system: HealthResolutionSystem,
    propagation_system: PropagationSystem,
    particle_emitter_system: ParticleEmitterSystem,
    particle_system: ParticleSystem,
    field_of_view_calculation_system: FieldOfViewCalculationSystem,
    world_maintenance_system: WorldMaintenanceSystem,
    save_load_system: SaveLoadSystem,

    //Player and UI variables
    symbolic_view: bool,

    current_tic: u32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut texture = Image::new(ctx, "/master8x8.png")?;
        texture.set_filter(FilterMode::Nearest);

        // Clear and create save directory
        let save_path = save_path();

        if save_path.exists() {
            fs::remove_dir_all(&save_path).unwrap();
        }

        fs::create_dir_all(&save_path).unwrap();

        //Register all components
        let mut ecs_world = ECSWorld::new();
        ecs_world.register::<AIActionComponent>();
        ecs_world.register::<AIGoalComponent>();
        ecs_world.register::<AIPerceptionComponent>();
        ecs_world.register::<AIPersonalityComponent>();
        ecs_world.register::<AttackComponent>();
        ecs_world.register::<ButcherableComponent>();
        ecs_world.register::<ColliderComponent>();
        ecs_world.register::<CollisionComponent>();
        ecs_world.register::<DeathComponent>();
        ecs_world.register::<DigestionComponent>();
        ecs_world.register::<DrawComponent>();
        ecs_world.register::<EdibleComponent>();
        ecs_world.register::<FieldOfViewComponent>();
        ecs_world.register::<HealthComponent>();
        ecs_world.register::<IdComponent>();
        ecs_world.register::<InputComponent>();
        ecs_world.register::<IntendedMovementComponent>();
        ecs_world.register::<InventoryComponent>();
        ecs_world.register::<ItemComponent>();
        ecs_world.register::<ManipulatorComponent>();
        ecs_world.register::<MaterialComponent>();
        ecs_world.register::<NameComponent>();
        ecs_world.register::<ParticleComponent>();
        ecs_world.register::<ParticleEmitterComponent>();
        ecs_world.register::<PathingComponent>();
        ecs_world.register::<PositionComponent>();
        ecs_world.register::<SaveMarkerComponent>();
        ecs_world.register::<SpeciesComponent>();
        ecs_world.register::<ToSaveComponent>();
        ecs_world.register::<VegPropagationComponent>();
        ecs_world.register::<VelocityComponent>();

        //Initialise all resources
        let keyboard = KeyboardResource {
            last_pressed_key: None,
            modifiers: KeyMods::default(),
        };

        let mouse = MouseResource {
            position: (0.0, 0.0),
            right_button_pressed: false,
            left_button_pressed: false,
        };

        let gen_package = GenPackageResource {
            elevation_noise: Perlin::new().set_seed(thread_rng().gen()),
            fertility_noise: Perlin::new().set_seed(thread_rng().gen()),
        };
        let mut tile_world = TileWorldResource::new(&gen_package, &mut ecs_world.system_data());
        let particle_map = ParticleMapResource::default();

        // TODO
        //let ui = UiResource { terminal: };

        //Insert pertinent data into resources
        //Needs braces to manually restrict scope of some data
        {
            let (lazy, entities, mut position, mut input): (
                Read<LazyUpdate>,
                Entities,
                WriteStorage<PositionComponent>,
                WriteStorage<InputComponent>,
            ) = ecs_world.system_data();

            let player = CreatureBuilder::Humanoid { species: Species::Human }.build(&lazy, &entities);
            // let player = CreatureBuilder::Deer { }.build(&lazy, &entities);
            input.insert(player, InputComponent::default()).unwrap();
            tile_world.spawn_entity(player, IPosition::new(16, 16), &mut position);
        }

        //Assign resources to ecs world
        ecs_world.insert(gen_package);
        ecs_world.insert(keyboard);
        ecs_world.insert(mouse);
        ecs_world.insert(tile_world);
        ecs_world.insert(particle_map);
        ecs_world.insert(IdGeneratorResource::new());
        ecs_world.insert(SaveMarkerAllocatorResource::new());
        ecs_world.insert(PendingLoadResource::new());
        ecs_world.insert(ViewportResource::new());
        ecs_world.insert(WeatherResource::new());

        let (char_width, char_height) = (8, 8);
        let (ui_width, ui_height) = (
            (WINDOW_WIDTH as f32 / (char_width as f32 * RENDER_SCALE)).floor() as usize,
            (WINDOW_HEIGHT as f32 / (char_height as f32 * RENDER_SCALE)).floor() as usize,
        );

        let save_load_system = SaveLoadSystem {
            bitset: BitSet::default(),
            to_save: Vec::new(),
            save_buf: Vec::new(),
        };

        ecs_world.maintain();

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
            weather_system: WeatherSystem,
            input_resolution_system: InputResolutionSystem,
            perception_resolution_system: PerceptionResolutionSystem,
            goal_resolution_system: GoalResolutionSystem,
            action_resolution_system: ActionResolutionSystem,
            collision_calculation_system: CollisionCalculationSystem,
            movement_resolution_system: MovementResolutionSystem,
            collision_resolution_system: CollisionResolutionSystem,
            digestion_resolution_system: DigestionResolutionSystem,
            health_resolution_system: HealthResolutionSystem,
            propagation_system: PropagationSystem,
            particle_emitter_system: ParticleEmitterSystem,
            particle_system: ParticleSystem,
            field_of_view_calculation_system: FieldOfViewCalculationSystem,
            world_maintenance_system: WorldMaintenanceSystem {
                save_buf: Vec::new(),
                ids: Vec::new(),
            },
            save_load_system,

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

        //Write resources
        {
            let data: InputData = self.ecs_world.system_data();
            let mut viewport = self.ecs_world.write_resource::<ViewportResource>();

            if let Some((_input, position)) = (&data.input, &data.position).join().next() {
                viewport.camera_world_position = position.pos;
            }
        }

        {
            let mut weather = self.ecs_world.write_resource::<WeatherResource>();
            weather.current_weather = WeatherStatus::Raining;
        }

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

        {
            let mut keyboard = self.ecs_world.write_resource::<KeyboardResource>();

            keyboard.last_pressed_key = keyboard::pressed_keys(ctx).iter().next().copied();
            keyboard.modifiers = keyboard::active_mods(ctx);
        }

        {
            let mut mouse = self.ecs_world.write_resource::<MouseResource>();

            let mouse_pos = mouse::position(ctx);
            mouse.position = (mouse_pos.x, mouse_pos.y);
            mouse.left_button_pressed = mouse::button_pressed(ctx, MouseButton::Left);
            mouse.right_button_pressed = mouse::button_pressed(ctx, MouseButton::Right);
        }

        self.ecs_world
            .write_resource::<ParticleMapResource>()
            .clear_all();

        //Run systems in order
        self.weather_system.run_now(&self.ecs_world);
        self.input_resolution_system.run_now(&self.ecs_world);
        self.perception_resolution_system.run_now(&self.ecs_world);
        self.goal_resolution_system.run_now(&self.ecs_world);
        self.action_resolution_system.run_now(&self.ecs_world);
        self.collision_calculation_system.run_now(&self.ecs_world);
        self.movement_resolution_system.run_now(&self.ecs_world);
        self.collision_resolution_system.run_now(&self.ecs_world);
        self.digestion_resolution_system.run_now(&self.ecs_world);
        self.health_resolution_system.run_now(&self.ecs_world);
        self.propagation_system.run_now(&self.ecs_world);
        self.particle_emitter_system.run_now(&self.ecs_world);
        self.particle_system.run_now(&self.ecs_world);
        self.field_of_view_calculation_system
            .run_now(&self.ecs_world);

        self.ecs_world.maintain();

        self.world_maintenance_system.run_now(&self.ecs_world);
        self.save_load_system.run_now(&self.ecs_world);

        self.ecs_world.maintain();

        self.current_tic += 1;

        std::thread::sleep(Duration::from_millis(50));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.05, 0.05, 0.05, 1.0].into());

        self.font_batch.clear();

        let data: RenderData = self.ecs_world.system_data();
        if let Some((
            input,
            position,
            field_of_view,
            inventory,
            manipulator,
            digestion,
            health,
            ai_goal,
        )) = (
            &data.input,
            &data.position,
            (&data.field_of_view).maybe(),
            (&data.inventory).maybe(),
            (&data.manipulator).maybe(),
            (&data.digestion).maybe(),
            (&data.health).maybe(),
            (&data.ai_goal).maybe(),
        )
            .join()
            .next()
        {
            let offset = MAP_X_SIZE as i32 / 2;
            let top_left = position.pos - IPosition::new(offset, offset);

            for (screen_y, particles) in
                (0..(MAP_Y_SIZE as i32)).zip_eq(data.particle_map.contents.iter())
            {
                for screen_x in 0..(MAP_X_SIZE as i32) {
                    let screen_pos = IPosition::new(screen_x, screen_y);
                    let world_pos = top_left + screen_pos;

                    let visible = if let Some(field_of_view) = field_of_view {
                        let radius = i32::try_from(field_of_view.shadowcast.radius()).unwrap();

                        let fov_pos = UPosition::try_from(
                            world_pos + IPosition::new(radius, radius) - position.pos,
                        )
                        .ok();

                        fov_pos
                            .and_then(|fov_pos| {
                                field_of_view
                                    .shadowcast
                                    .fov()
                                    .get(fov_pos.to_idx().unwrap())
                                    .copied()
                            })
                            .unwrap_or(false)
                    } else {
                        true
                    };

                    if visible {
                        if let Some(tile) = data.tile_world.get(world_pos) {
                            if self.symbolic_view {
                                tile.tile
                                    .get_symbolbuilder()
                                    .get_symbol(tile.tile.seed)
                                    .draw_to_font_batch(
                                        &mut self.font_batch,
                                        (screen_x, screen_y),
                                        RENDER_SCALE,
                                    );
                            } else {
                                tile.tile
                                    .get_spritebuilder()
                                    .get_sprite(tile.tile.seed)
                                    .draw_to_font_batch(
                                        &mut self.font_batch,
                                        (screen_x, screen_y),
                                        RENDER_SCALE,
                                    );
                            }

                            for entity in tile.entities.iter() {
                                let dc = data.draw.get(*entity).unwrap();

                                if self.symbolic_view {
                                    if let Some(sym_build) = &dc.symbol_builder {
                                        sym_build
                                            .get_symbol(entity.id() as usize)
                                            .draw_to_font_batch(
                                                &mut self.font_batch,
                                                (screen_x, screen_y),
                                                RENDER_SCALE,
                                            );
                                    }
                                } else {
                                    dc.sprite_builder
                                        .get_sprite(entity.id() as usize)
                                        .draw_to_font_batch(
                                            &mut self.font_batch,
                                            (screen_x, screen_y),
                                            RENDER_SCALE,
                                        );
                                }
                            }
                        }
                    }
                }

                for particle in particles.iter() {
                    let pac = data.particle.get(*particle).unwrap();

                    pac.particle_type.get_char().draw_to_font_batch(
                        &mut self.font_batch,
                        (pac.position - top_left - IPosition::new(0, pac.height)).as_tuple(),
                        RENDER_SCALE,
                    );
                }

                if let Some(path) = &input.path {
                    for step in path {
                        GgBunnyChar {
                            index: b'.' as usize,
                            foreground: Color::new(0.75, 0.75, 0.0, 1.0),
                            background: None,
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }
                        .draw_to_font_batch(
                            &mut self.font_batch,
                            (*step - top_left).as_tuple(),
                            RENDER_SCALE,
                        );
                    }
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
                        let (digestion_pane, rest) = Layout::default()
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

                        right_pane = rest;

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

                    if let Some(manipulator) = manipulator {
                        let (manipulator_pane, rest) = Layout::default()
                            .direction(LayoutDirection::Vertical)
                            .constraints([Constraint::Length(3), Constraint::Min(0)])
                            .split(right_pane)
                            .into_iter()
                            .collect_tuple()
                            .unwrap();

                        right_pane = rest;

                        let list = List::new(vec![if let Some(item) = &manipulator.held_item {
                            ListItem::new(data.name.get(*item).unwrap().name.as_str())
                        } else {
                            ListItem::new("-")
                        }]);

                        let block = Block::default().title("Held").borders(Borders::ALL);

                        f.render_widget(list.block(block), manipulator_pane);
                    }

                    if let Some(ai_goal) = ai_goal {
                        let (ai_goal_pane, _rest) = Layout::default()
                            .direction(LayoutDirection::Vertical)
                            .constraints([
                                Constraint::Length(
                                    (ai_goal.goal_stack.len() as u16).min(10).max(2) + 2,
                                ),
                                Constraint::Min(0),
                            ])
                            .split(right_pane)
                            .into_iter()
                            .collect_tuple()
                            .unwrap();

                        let list = List::new(
                            ai_goal
                                .goal_stack
                                .iter()
                                .rev()
                                .map(|goal| ListItem::new(goal.get_textual_representation(&data)))
                                .collect::<Vec<_>>(),
                        );

                        let block = Block::default().title("Goals").borders(Borders::ALL);

                        f.render_widget(list.block(block), ai_goal_pane);
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
                })
                .unwrap();
        }

        ggez::graphics::draw(ctx, &self.font_batch, DrawParam::default())?;
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

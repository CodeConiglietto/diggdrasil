use bunnyfont::{
    char_transforms::{CharMirror, CharRotation},
    ggez::GgBunnyChar,
};
use ggez::graphics::Color;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

//TODO: Maybe change these to be more generic if there's too much code repetition
//TODO: Create some method to instantiate these to remove potential for creating malformed particles
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ParticleType {
    Finished,
    Rain { initial_angle: Direction },
    RainSplash { lifetime: usize },
    // Snow { y_cutoff: usize },
    Leaf,
    Blood { x_vel: i32, y_vel: i32, z_vel: i32 },
    // Splinter { direction: Direction },
    // Debris { direction: Direction },
    Smoke { color_value: f32, lifetime: usize },
    Thrust { drawn: bool, direction_from_player: Direction }, 
    Swing { drawn: bool, direction_from_player: Direction, rotation_direction: RotationDirection },
}

impl ParticleType {
    pub fn get_new_position(&self, (x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        match self {
            Self::Rain { .. } => (x - 1, y, z - 1),
            Self::RainSplash { .. } => (x, y, z),
            Self::Leaf => (
                x + thread_rng().gen_range(-1..=1),
                y + thread_rng().gen_range(-1..=1),
                z - thread_rng().gen_range(0..=1),
            ),
            Self::Smoke { .. } => (
                x + if thread_rng().gen_range(0..=4) == 0 {
                    thread_rng().gen_range(-1..=1)
                } else {
                    0
                },
                y + if thread_rng().gen_range(0..=4) == 0 {
                    thread_rng().gen_range(-1..=1)
                } else {
                    0
                },
                z + if thread_rng().gen_range(0..=3) != 0 {
                    1
                } else {
                    0
                },
            ),
            Self::Blood { x_vel, y_vel, z_vel } => (x + x_vel, y + y_vel, z + z_vel),
            Self::Thrust { .. } => (x, y, z),
            Self::Swing { .. } => (x, y, z),
            _ => todo!(),
        }
    }

    //change this to use an option with none if there are no changes
    pub fn get_new_state(
        &self,
        (x, y, z): (i32, i32, i32),
        (player_x, player_y): (i32, i32),
    ) -> ParticleType {
        let left = player_x - MAP_X_SIZE as i32 / 2;
        let right = left + MAP_X_SIZE as i32;
        let top = player_y - MAP_Y_SIZE as i32 / 2;
        let bottom = top + MAP_Y_SIZE as i32;

        if !(left..right).contains(&x)
            || !(top..bottom).contains(&y)
            || !(0..=MAX_PARTICLE_HEIGHT).contains(&z)
        {
            return Self::Finished;
        }

        match self {
            Self::Rain { .. } => {
                if z == 1 {
                    ParticleType::RainSplash { lifetime: 0 }
                } else {
                    *self
                }
            }
            Self::RainSplash { lifetime } => {
                if *lifetime >= 3 {
                    ParticleType::Finished
                } else {
                    ParticleType::RainSplash {
                        lifetime: lifetime + 1,
                    }
                }
            }
            Self::Leaf => {
                if z == 0 {
                    ParticleType::Finished
                } else {
                    *self
                }
            }
            Self::Smoke { color_value, lifetime } => {
                if z == MAX_PARTICLE_HEIGHT || *lifetime >= 23 {
                    ParticleType::Finished
                } else {
                    ParticleType::Smoke {
                        color_value: color_value + thread_rng().gen_range(0.0..0.1) - 0.05,
                        lifetime: lifetime + 1,
                    }
                }
            }
            Self::Blood { x_vel, y_vel, z_vel } => {
                if z == 0 {
                    ParticleType::Finished
                } else {
                    let x_vel = 
                        if *z_vel == -1 && thread_rng().gen::<bool>() {
                            0
                        } else {
                            *x_vel
                        };
                    let y_vel = 
                        if *z_vel == -1 && thread_rng().gen::<bool>() {
                            0
                        } else {
                            *y_vel
                        };
                    let z_vel = 
                        if *z_vel >= 0 && thread_rng().gen::<bool>() {
                            *z_vel - 1
                        } else {
                            *z_vel
                        };

                    ParticleType::Blood {
                        x_vel,
                        y_vel,
                        z_vel,
                    }
                }
            }
            Self::Thrust { drawn, direction_from_player } => if *drawn { ParticleType::Finished } else { Self::Thrust{drawn: true, direction_from_player: *direction_from_player} },
            Self::Swing { drawn, direction_from_player, rotation_direction } => if *drawn { ParticleType::Finished } else { Self::Swing{drawn: true, direction_from_player: *direction_from_player, rotation_direction: *rotation_direction} },
            _ => *self,
        }
    }

    pub fn get_char(&self) -> GgBunnyChar {
        match self {
            Self::Rain { .. } => GgBunnyChar {
                index: 0x11F,
                foreground: Color::new(0.0, 0.0, 0.75, 1.0),
                background: None,
                rotation: CharRotation::None,
                mirror: CharMirror::None,
            },
            Self::RainSplash { lifetime } => {
                let splash_anim = [
                    GgBunnyChar {
                        index: 0x189,
                        foreground: Color::new(0.0, 0.0, 0.75, 1.0),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::MirrorX,
                    },
                    GgBunnyChar {
                        index: 0x15F,
                        foreground: Color::new(0.0, 0.0, 0.75, 1.0),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::MirrorX,
                    },
                    GgBunnyChar {
                        index: 0x13F,
                        foreground: Color::new(0.0, 0.0, 0.75, 1.0),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::MirrorX,
                    },
                    GgBunnyChar {
                        index: 0x009,
                        foreground: Color::new(0.0, 0.0, 0.75, 1.0),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::MirrorX,
                    },
                ];

                splash_anim[*lifetime]
            }
            Self::Leaf => GgBunnyChar {
                index: 0x060,
                foreground: Color::new(0.0, 0.75, 0.0, 1.0),
                background: None,
                rotation: CharRotation::Rotation270,
                mirror: if thread_rng().gen::<bool>() {
                    CharMirror::None
                } else {
                    CharMirror::MirrorX
                },
            },
            Self::Smoke { color_value, lifetime } => {
                let index = if *lifetime < 16 {
                    0x390 + lifetime
                } else {
                    0x2B7 - (lifetime - 16)
                };

                GgBunnyChar {
                    index,
                    foreground: Color::new(*color_value, *color_value, *color_value, 1.0),
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                }
            },
            Self::Blood { .. } =>
                GgBunnyChar {
                    index: 0x391,
                    foreground: Color::new(1.0, 0.0, 0.0, 1.0),
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                },
            Self::Thrust { direction_from_player, .. } => {
                let (index, rotation) = match direction_from_player {
                    Direction::None => (0x00F, Rotation::None),
                    Direction::UpLeft => (0x30D, Rotation::None),
                    Direction::Up => (0x05E, Rotation::None),
                    Direction::UpRight => (0x30D, Rotation::Rotation90),
                    Direction::Right => (0x05E, Rotation::Rotation90),
                    Direction::DownRight => (0x30D, Rotation::Rotation180),
                    Direction::Down => (0x05E, Rotation::Rotation180),
                    Direction::DownLeft => (0x30D, Rotation::Rotation270),
                    Direction::Left => (0x05E, Rotation::Rotation270),
                };

                GgBunnyChar {
                    index,
                    foreground: Color::new(0.75, 0.75, 0.75, 1.0),
                    background: None,
                    rotation, 
                    mirror: CharMirror::None,
                }
            }
            Self::Swing { direction_from_player, .. } => {
                let (index, rotation) = match direction_from_player {
                    Direction::None => (0x00F, Rotation::None),
                    Direction::UpLeft => (0x30D, Rotation::None),
                    Direction::Up => (0x05E, Rotation::None),
                    Direction::UpRight => (0x30D, Rotation::Rotation90),
                    Direction::Right => (0x05E, Rotation::Rotation90),
                    Direction::DownRight => (0x30D, Rotation::Rotation180),
                    Direction::Down => (0x05E, Rotation::Rotation180),
                    Direction::DownLeft => (0x30D, Rotation::Rotation270),
                    Direction::Left => (0x05E, Rotation::Rotation270),
                };

                GgBunnyChar {
                    index,
                    foreground: Color::new(0.75, 0.75, 0.75, 1.0),
                    background: None,
                    rotation,
                    mirror: CharMirror::None,
                }
            }
            _ => todo!("{:?}", self),
        }
    }
}

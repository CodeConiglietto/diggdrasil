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
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ParticleType {
    Finished,
    Rain { initial_angle: Direction },
    RainSplash { lifetime: usize },
    // Snow { y_cutoff: usize },
    Leaf,
    // Blood { direction: Direction },
    // Splinter { direction: Direction },
    // Debris { direction: Direction },
    Smoke { lifetime: usize },
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
                if z == 1 {
                    ParticleType::Finished
                } else {
                    *self
                }
            }
            Self::Smoke { lifetime } => {
                if z == MAX_PARTICLE_HEIGHT || *lifetime >= 23 {
                    ParticleType::Finished
                } else {
                    ParticleType::Smoke {
                        lifetime: lifetime + 1,
                    }
                }
            }
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
            Self::Smoke { lifetime } => {
                let index = if *lifetime < 16 {
                    0x390 + lifetime
                } else {
                    0x2B7 - (lifetime - 16)
                };

                GgBunnyChar {
                    index,
                    foreground: Color::new(0.5, 0.5, 0.5, 1.0),
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                }
            }
            _ => todo!(),
        }
    }
}

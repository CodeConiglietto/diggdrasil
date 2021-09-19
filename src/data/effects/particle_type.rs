use bunnyfont::{
    char::BunnyChar,
    char_transforms::{CharMirror, CharRotation},
};
use rand::prelude::*;
use ggez::graphics::Color;

use crate::prelude::*;

//Maybe change these to be more generic if there's too much code repetition
#[derive(Clone, Copy)]
pub enum ParticleType {
    Finished,
    Rain { initial_angle: Direction },
    RainSplash { lifetime: usize },
    // Snow { y_cutoff: usize },
    Leaf,
    // Blood { direction: Direction },
    // Splinter { direction: Direction },
    // Debris { direction: Direction },
    // Smoke {lifetime: usize, color: DiggColor},
}

impl ParticleType {
    pub fn get_new_position(&self, (x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        match self {
            Self::Rain { .. } => (x - 1, y, z - 1),
            Self::RainSplash { .. } => (x, y, z),
            Self::Leaf => (x + thread_rng().gen_range(-1..=1), y + thread_rng().gen_range(-1..=1), z - 1),
            _ => todo!(),
        }
    }

    //change this to use an option with none if there are no changes
    pub fn get_new_state(&self, (x, y, z): (i32, i32, i32)) -> ParticleType {
        if x < 0
            || x >= MAP_X_SIZE as i32
            || y < 0
            || y >= MAP_Y_SIZE as i32
            || z < 0
            || z > MAX_PARTICLE_HEIGHT
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
            _ => *self,
        }
    }

    pub fn get_char(&self) -> DiggChar {
        match self {
            Self::Rain { .. } => DiggChar {
                inner: BunnyChar {
                    index: 0x11F,
                    foreground: DiggColor {
                        inner: Color::new(0.0, 0.0, 0.75, 1.0),
                    },
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::MirrorX,
                },
            },
            Self::RainSplash { lifetime } => {
                let splash_anim = [
                    DiggChar {
                        inner: BunnyChar {
                            index: 0x189,
                            foreground: DiggColor {
                                inner: Color::new(0.0, 0.0, 0.75, 1.0),
                            },
                            background: None,
                            rotation: CharRotation::None,
                            mirror: CharMirror::MirrorX,
                        },
                    },
                    DiggChar {
                        inner: BunnyChar {
                            index: 0x15F,
                            foreground: DiggColor {
                                inner: Color::new(0.0, 0.0, 0.75, 1.0),
                            },
                            background: None,
                            rotation: CharRotation::None,
                            mirror: CharMirror::MirrorX,
                        },
                    },
                    DiggChar {
                        inner: BunnyChar {
                            index: 0x13F,
                            foreground: DiggColor {
                                inner: Color::new(0.0, 0.0, 0.75, 1.0),
                            },
                            background: None,
                            rotation: CharRotation::None,
                            mirror: CharMirror::MirrorX,
                        },
                    },
                    DiggChar {
                        inner: BunnyChar {
                            index: 0x009,
                            foreground: DiggColor {
                                inner: Color::new(0.0, 0.0, 0.75, 1.0),
                            },
                            background: None,
                            rotation: CharRotation::None,
                            mirror: CharMirror::MirrorX,
                        },
                    },
                ];

                splash_anim[*lifetime]
            },
            Self::Leaf => {
                DiggChar {
                    inner: BunnyChar {
                        index: 0x060,
                        foreground: DiggColor {
                            inner: Color::new(0.0, 0.75, 0.0, 1.0),
                        },
                        background: None,
                        rotation: CharRotation::Rotation90,
                        mirror: if thread_rng().gen::<bool>() { CharMirror::None } else { CharMirror::MirrorX },
                    }
                }
            }
            _ => todo!(),
        }
    }
}

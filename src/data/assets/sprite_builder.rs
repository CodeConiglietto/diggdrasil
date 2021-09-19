use bunnyfont::{
    char::BunnyChar,
    char_transforms::{CharMirror, CharRotation},
};

use ggez::graphics::Color;
use ndarray::prelude::*;

use crate::prelude::*;

pub enum SpriteBuilder {
    Wall { seed: usize },
    Ground { seed: usize },
    Humanoid { race: Race },
    Tree,
    Log,
}

impl SpriteBuilder {
    pub fn get_sprite(&self) -> Sprite {
        match self {
            Self::Wall { seed } => Sprite {
                origin_x: 0,
                origin_y: 1,
                contents: array![
                    [DiggChar {
                        inner: BunnyChar {
                            index: 0x321,
                            foreground: DiggColor {
                                inner: Color::new(0.25, 0.25, 0.25, 1.0)
                            },
                            background: Some(DiggColor {
                                inner: Color::new(0.0, 0.0, 0.0, 1.0)
                            }),
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }
                    }],
                    [DiggChar {
                        inner: BunnyChar {
                            index: 0x2B3 + (seed) % 3,
                            foreground: DiggColor {
                                inner: Color::new(0.4, 0.4, 0.4, 1.0)
                            },
                            background: Some(DiggColor {
                                inner: Color::new(0.25, 0.25, 0.25, 1.0)
                            }),
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }
                    }],
                    [DiggChar {
                        inner: BunnyChar {
                            index: 0x2B0 + (seed) % 8,
                            foreground: DiggColor {
                                inner: Color::new(0.2, 0.15, 0.15, 1.0)
                            },
                            background: Some(DiggColor {
                                inner: Color::new(0.15, 0.1, 0.1, 1.0)
                            }),
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }
                    }]
                ],
            },
            Self::Ground { seed } => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![
                    [DiggChar {
                        inner: BunnyChar {
                            index: 0x2B1 + (seed) % 6,
                            foreground: DiggColor {
                                inner: Color::new(0.2, 0.25, 0.2, 1.0)
                            },
                            background: Some(DiggColor {
                                inner: Color::new(0.25, 0.2, 0.2, 1.0)
                            }),
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }
                    }],
                    [DiggChar {
                        inner: BunnyChar {
                            index: 0x2B4 + (seed) % 4,
                            foreground: DiggColor {
                                inner: Color::new(0.2, 0.15, 0.15, 1.0)
                            },
                            background: Some(DiggColor {
                                inner: Color::new(0.15, 0.1, 0.1, 1.0)
                            }),
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }
                    }]
                ],
            },
            Self::Humanoid { race } => Sprite {
                origin_x: 0,
                origin_y: 1,
                contents: array![
                    [DiggChar {
                        inner: BunnyChar {
                            index: 0x00C,
                            foreground: race.get_color(),
                            background: None,
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }
                    }],
                    [DiggChar {
                        inner: BunnyChar {
                            index: 0x05E,
                            foreground: race.get_color(),
                            background: None,
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }
                    }]
                ],
            },
            Self::Tree => Sprite {
                origin_x: 0,
                origin_y: 1,
                contents: array![
                    [DiggChar {
                        inner: BunnyChar {
                            index: 0x02A,
                            foreground: DiggColor {
                                inner: Color::new(0.0, 1.0, 0.0, 1.0)
                            },
                            background: None,
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }
                    }],
                    [DiggChar {
                        inner: BunnyChar {
                            index: 0x07C,
                            foreground: DiggColor {
                                inner: Color::new(0.5, 0.5, 0.0, 1.0)
                            },
                            background: None,
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }
                    }]
                ],
            },
            Self::Log => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[DiggChar {
                    inner: BunnyChar {
                        index: 0x357,
                        foreground: DiggColor {
                            inner: Color::new(0.75, 0.75, 0.0, 1.0)
                        },
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    }
                }],],
            },
        }
    }

    pub fn get_symbol(&self) -> DiggChar {
        match self {
            Self::Wall { .. } => DiggChar {
                inner: BunnyChar {
                    index: 0x321,
                    foreground: DiggColor {
                        inner: Color::new(0.25, 0.25, 0.25, 1.0),
                    },
                    background: Some(DiggColor {
                        inner: Color::new(0.0, 0.0, 0.0, 1.0),
                    }),
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                },
            },
            Self::Ground { seed } => DiggChar {
                inner: BunnyChar {
                    index: 0x2B1 + (seed) % 6,
                    foreground: DiggColor {
                        inner: Color::new(0.2, 0.25, 0.2, 1.0),
                    },
                    background: Some(DiggColor {
                        inner: Color::new(0.25, 0.2, 0.2, 1.0),
                    }),
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                },
            },
            Self::Humanoid { race } => DiggChar {
                inner: BunnyChar {
                    index: race.get_symbol(),
                    foreground: DiggColor {
                        inner: Color::new(1.0, 0.0, 0.0, 1.0),
                    },
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                },
            },
            Self::Tree => DiggChar {
                inner: BunnyChar {
                    index: 0x005,
                    foreground: DiggColor {
                        inner: Color::new(0.0, 1.0, 0.0, 1.0),
                    },
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                },
            },
            Self::Log => DiggChar {
                inner: BunnyChar {
                    index: 0x357,
                    foreground: DiggColor {
                        inner: Color::new(0.75, 0.75, 0.0, 1.0),
                    },
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                },
            },
        }
    }
}

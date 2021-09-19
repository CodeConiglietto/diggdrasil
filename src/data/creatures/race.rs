use ggez::graphics::Color;

#[derive(Clone, Copy)]
pub enum Race {
    Human,
    Elf,
    Kobold,
    Goblin,
}

impl Race {
    pub fn get_name(&self) -> &str {
        match self {
            Self::Human => "human",
            Self::Elf => "elf",
            Self::Kobold => "kobold",
            Self::Goblin => "goblin",
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Self::Human => Color::new(0.75, 0.0, 0.0, 1.0),
            Self::Elf => Color::new(0.0, 0.75, 0.0, 1.0),
            Self::Kobold => Color::new(0.75, 0.75, 0.75, 1.0),
            Self::Goblin => Color::new(0.75, 0.75, 0.0, 1.0),
        }
    }

    pub fn get_symbol(&self) -> usize {
        let character = self.get_name().chars().next().unwrap();

        assert!(character.is_ascii_alphabetic());

        u32::from(character) as usize
    }
}

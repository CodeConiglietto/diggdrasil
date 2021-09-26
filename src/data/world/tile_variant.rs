use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct TileVariant {
    pub layout: TileLayout,
    pub rotation: Rotation,
}

impl TileVariant {
    pub fn get_from_neighbours(
        (u, d, l, r): (
            Option<TileType>,
            Option<TileType>,
            Option<TileType>,
            Option<TileType>,
        ),
    ) -> TileVariant {
        let neighbours = (
            if let Some(u) = u { u.connects() } else { false },
            if let Some(d) = d { d.connects() } else { false },
            if let Some(l) = l { l.connects() } else { false },
            if let Some(r) = r { r.connects() } else { false },
        );

        //This makes me sad
        //TODO: find a more elegant way to do this, if that's even possible
        let (layout, rotation) = match neighbours {
            (false, false, false, false) => (TileLayout::Pillar, Rotation::None),
            (false, false, false, true) => (TileLayout::Single, Rotation::Rotation180),
            (false, false, true, false) => (TileLayout::Single, Rotation::None),
            (false, false, true, true) => (TileLayout::Straight, Rotation::None),
            (false, true, false, false) => (TileLayout::Single, Rotation::Rotation270),
            (false, true, false, true) => (TileLayout::Corner, Rotation::None),
            (false, true, true, false) => (TileLayout::Corner, Rotation::Rotation90),
            (false, true, true, true) => (TileLayout::Three, Rotation::Rotation90),
            (true, false, false, false) => (TileLayout::Single, Rotation::Rotation90),
            (true, false, false, true) => (TileLayout::Corner, Rotation::Rotation270),
            (true, false, true, false) => (TileLayout::Corner, Rotation::Rotation180),
            (true, false, true, true) => (TileLayout::Three, Rotation::Rotation270),
            (true, true, false, false) => (TileLayout::Straight, Rotation::Rotation270),
            (true, true, false, true) => (TileLayout::Three, Rotation::None),
            (true, true, true, false) => (TileLayout::Three, Rotation::Rotation180),
            (true, true, true, true) => (TileLayout::All, Rotation::None),
        };

        TileVariant { layout, rotation }
    }

    pub fn get_occupied_sides(&self) -> (bool, bool) {
        match self.layout {
            TileLayout::All => (true, true),
            TileLayout::Three => match self.rotation {
                Rotation::None => (false, true),
                Rotation::Rotation180 => (true, false),
                Rotation::Rotation90 | Rotation::Rotation270 => (true, true),
            },
            TileLayout::Straight => match self.rotation {
                Rotation::None | Rotation::Rotation180 => (true, true),
                Rotation::Rotation90 | Rotation::Rotation270 => (false, false),
            },
            TileLayout::Corner => match self.rotation {
                Rotation::None | Rotation::Rotation270 => (false, true),
                Rotation::Rotation90 | Rotation::Rotation180 => (true, false),
            },
            TileLayout::Single => match self.rotation {
                Rotation::Rotation90 | Rotation::Rotation270 => (false, false),
                Rotation::None => (true, false),
                Rotation::Rotation180 => (false, true),
            },
            TileLayout::Pillar => (false, false),
        }
    }

    //TODO: Make a structure for a colorless char to put in here
    pub fn get_top_fill_char(&self) -> (usize, Rotation, Mirror) {
        //TODO: maybe find a better way to do this?
        match self.get_occupied_sides() {
            (false, false) => (0x32F, Rotation::Rotation270, Mirror::None),
            (false, true) => (0x2CC, Rotation::None, Mirror::None),
            (true, false) => (0x2CC, Rotation::None, Mirror::MirrorX),
            (true, true) => (0x2C4, Rotation::None, Mirror::None),
        }
    }

    //TODO: Make a structure for a colorless char to put in here
    pub fn get_mid_char(
        &self,
        (ind, rot, mir): (usize, Rotation, Mirror),
    ) -> (usize, Rotation, Mirror) {
        //TODO: maybe find a better way to do this?
        match self.get_occupied_sides() {
            (false, false) => (0x2F5, Rotation::Rotation90, Mirror::None),
            (false, true) => (0x2C4, Rotation::Rotation270, Mirror::None),
            (true, false) => (0x2C4, Rotation::Rotation90, Mirror::None),
            (true, true) => (ind, rot, mir),
        }
    }
}

impl Default for TileVariant {
    fn default() -> TileVariant {
        TileVariant {
            layout: TileLayout::default(),
            rotation: Rotation::None,
        }
    }
}

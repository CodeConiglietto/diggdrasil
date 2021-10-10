use std::convert::TryFrom;

use specs::prelude::*;

use crate::prelude::*;

pub struct FieldOfViewCalculationSystem;

impl<'a> System<'a> for FieldOfViewCalculationSystem {
    type SystemData = (
        ReadExpect<'a, TileWorldResource>,
        ReadStorage<'a, PositionComponent>,
        WriteStorage<'a, FieldOfViewComponent>,
        WriteStorage<'a, AIPerceptionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (twld, pos, mut fov, mut apc) = data;

        for (pos, fov, mut apc) in (&pos, &mut fov, (&mut apc).maybe()).join() {
            if let Some(apc) = &mut apc {
                apc.all.clear();
            }

            fov.shadowcast.shadowcast(&mut FieldOfViewCallbacks {
                radius: fov.shadowcast.radius(),
                tile_world: &*twld,
                position: pos,
                ai_perception: apc,
            });
        }
    }
}

struct FieldOfViewCallbacks<'a> {
    radius: u32,
    tile_world: &'a TileWorldResource,
    position: &'a PositionComponent,
    ai_perception: Option<&'a mut AIPerceptionComponent>,
}

impl<'a> FieldOfViewCallbacks<'a> {
    fn fov_to_world_pos(&self, fov_pos: UPosition) -> IPosition {
        IPosition::try_from(fov_pos).unwrap()
            - IPosition::try_from(UPosition::new(self.radius, self.radius)).unwrap()
            + self.position.pos
    }
}

impl<'a> ShadowcastCallbacks for FieldOfViewCallbacks<'a> {
    fn is_visible(&mut self, fov_pos: UPosition) -> bool {
        let pos = self.fov_to_world_pos(fov_pos);

        if let Some(chunk_tile) = self.tile_world.get(pos) {
            !chunk_tile.tile.tile_type.collides()
        } else {
            false
        }
    }

    fn on_visible(&mut self, fov_pos: UPosition) {
        let pos = self.fov_to_world_pos(fov_pos);

        if let Some(ai_perception) = &mut self.ai_perception {
            if let Some(chunk_tile) = self.tile_world.get(pos) {
                for entity in chunk_tile.entities.iter() {
                    ai_perception.all.push(*entity);
                }
            }
        }
    }
}

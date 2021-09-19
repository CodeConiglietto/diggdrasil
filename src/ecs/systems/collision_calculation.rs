use crate::prelude::*;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct CollisionCalculationSystem;

impl<'a> System<'a> for CollisionCalculationSystem {
    type SystemData = (
        Read<'a, TileMapResource>,
        Read<'a, EntityMapResource>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, IntendedMovementComponent>,
        ReadStorage<'a, ColliderComponent>,
        WriteStorage<'a, CollisionComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (tmap, emap, pos, imc, coc, mut col) = data;

        for (pos, imc, col) in (&pos, &imc, &mut col).join() {
            if imc.x_delta != 0 || imc.y_delta != 0 {
                let (new_x, new_y) = (pos.x + imc.x_delta, pos.y + imc.y_delta);

                let tile = &tmap.contents[[new_x as usize, new_y as usize]];
                let entities = &emap.contents[[new_x as usize, new_y as usize]];

                if tile.tile_type.collides() {
                    col.tile_collision = Some(*tile);
                }

                for entity in entities {
                    if coc.get(*entity).is_some() {
                        col.entity_collisions.push(*entity);
                        break;
                    }
                }

                println!("Colliding with tile? {}", col.tile_collision.is_some());
                println!("Colliding with {} entities", col.entity_collisions.len());
            }
        }
    }
}

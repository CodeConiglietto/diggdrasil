use specs::prelude::*;

use crate::prelude::*;

pub struct BuildGoal {
    //Child goals and data here
    pos: Option<(i32, i32)>,
    tile_type: Option<TileType>,
    consumed_entity: Option<Entity>,
}

impl AIGoalTrait for BuildGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        let (x, y) = data.position.get(parent_entity).unwrap();

        if let Some(chunk_tile) = data.tile_world.get((*x, *y)) {
            if let Some(inv) = data.inventory.get(parent_entity) {
                if let Some(tile_type) = self.tile_type {
                    if let Some(consumed_entity) = self.consumed_entity {
                        Self::action(AIAction::BuildAtLocation {
                            x: *x,
                            y: *y,
                            tile_type: tile_type,
                            consumed_entity: consumed_entity,
                        })
                    } else {
                        panic!()
                        // if let Some(inp) = inp {
                        //     let item_goals = inv
                        //         .items
                        //         .iter()
                        //         .enumerate()
                        //         .filter_map(|(i, slot)| {
                        //             slot.and_then(|item| {
                        //                 if let Some(material) =
                        //                     crd.material.get(item)
                        //                 {
                        //                     if fulfills_material_requirements(
                        //                         material,
                        //                         tile_type.get_build_requirements(),
                        //                     ) {
                        //                         Some(PopupListItem::from((
                        //                             i,
                        //                             if let Some(item_name) =
                        //                                 nam.get(item)
                        //                             {
                        //                                 Some(item_name.name.clone())
                        //                             } else {
                        //                                 None
                        //                             },
                        //                             AIGoal::Build {
                        //                                 x: *x,
                        //                                 y: *y,
                        //                                 tile_type: Some(*tile_type),
                        //                                 consumed_entity: Some(item),
                        //                             },
                        //                         )))
                        //                     } else {
                        //                         None
                        //                     }
                        //                 } else {
                        //                     None
                        //                 }
                        //             })
                        //         })
                        //         .collect();

                        //     inp.popup = Some(Popup::list(
                        //         format!("Build with what?",),
                        //         item_goals,
                        //     ));
                        // } else {
                        //     println!("Entity trying to find building material doesn't have input component");
                        // }
                    }
                } else {
                    panic!()
                    // if let Some(inp) = inp {
                    //     let available_materials: Vec<_> = inv
                    //         .items
                    //         .iter()
                    //         .filter_map(|slot| {
                    //             if let Some(item) = slot {
                    //                 if let Some(material) = crd.material.get(*item)
                    //                 {
                    //                     Some(material)
                    //                 } else {
                    //                     None
                    //                 }
                    //             } else {
                    //                 None
                    //             }
                    //         })
                    //         .collect();

                    //     let tile_goals = chunk_tile
                    //         .tile
                    //         .tile_type
                    //         .available_buildings()
                    //         .iter()
                    //         .filter(|building| {
                    //             let build_requirements =
                    //                 building.get_build_requirements();

                    //             for available_material in &available_materials {
                    //                 if fulfills_material_requirements(
                    //                     available_material,
                    //                     build_requirements,
                    //                 ) {
                    //                     return true;
                    //                 }
                    //             }

                    //             false
                    //         })
                    //         .enumerate()
                    //         .map(|(i, tile_type)| {
                    //             (
                    //                 i,
                    //                 Some(String::from(tile_type.get_name())),
                    //                 AIGoal::Build {
                    //                     x: *x,
                    //                     y: *y,
                    //                     tile_type: Some(*tile_type),
                    //                     consumed_entity: consumed_entity.clone(),
                    //                 },
                    //             )
                    //         })
                    //         .map(PopupListItem::from)
                    //         .collect();

                    //     inp.popup =
                    //         Some(Popup::list(format!("Build what?"), tile_goals));
                    // } else {
                    //     println!(
                    //         "Entity trying to decide building doesn't have input component"
                    //     );
                    // }
                }
            } else {
                println!("Entity trying to find building material doesn't have inventory component");
                panic!()
            }
        } else {
            println!("Entity trying to build in an unloaded tile!");
            panic!()
        }
    }
}
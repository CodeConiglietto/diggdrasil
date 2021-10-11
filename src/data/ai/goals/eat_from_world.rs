
    use specs::prelude::*;

    use crate::prelude::*;
    
    pub struct EatFromWorldGoal {
        //Child goals and data here
        pub target: Entity,
        pub move_to_entity_goal: Option<MoveToEntityGoal>,
    }
    
    impl AIGoalTrait for EatFromWorldGoal {
        fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
            let this_pos = data.pos.get(parent_entity).unwrap();
            
            if let Some(target_pos) = data.position.get(self.target) {
                //We can assume that the item exists in the world
                if pos_is_adjacent(this_pos.get_pos_tuple(), target_pos.get_pos_tuple(), true) {
                    Self::action(AIAction::EatFromGround { target: self.target })
                } else {
                    if !self.move_to_entity_goal
                        .get_or_insert_with(
                            || MoveToEntityGoal{
                                target: self.target
                            }
                            )
                            .resolve(parent_entity, data)? 
                    {
                        return Self::failure()
                    }

                    //This may be the case, if not rethink everything
                    unreachable!();
                }
            } else {
                println!("Entity attempting to eat item that is not in the world!");
                Self::failure()
            }
        }
    }
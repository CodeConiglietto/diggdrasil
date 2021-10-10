use specs::prelude::*;

use crate::prelude::*;

pub struct HoldItemGoal {
    //Child goals and data here
}

impl AIGoalTrait for HoldItemGoal {
    fn resolve(&mut self, parent_entity: Entity, data: GoalData) -> AIGoalResult {
        if let Some(man) = man {
            if let Some(held) = man.held_item {
                //TODO: make this stow, drop, or sheath the held item
                AIGoalStatus::HasChildGoals {
                    goals: vec![AIGoal::StowItem { item: held }],
                }
            } else {
                if let Some(item) = item {
                    Self::action(AIAction::HoldItemFromInventory { item: *item });
                } else {
                    if let Some(inp) = inp {
                        if let Some(inv) = inv {
                            let item_goals = inv
                                .items
                                .iter()
                                .enumerate()
                                .filter_map(|(i, slot)| {
                                    slot.map(|item| {
                                        (
                                            i,
                                            None,
                                            AIGoal::HoldItem { item: Some(item) },
                                        )
                                    })
                                })
                                .map(PopupListItem::from)
                                .collect();

                            inp.popup = Some(Popup::list(
                                format!("Hold what?",),
                                item_goals,
                            ));
                        }
                    }
                }
                Self::success()
            }
        } else {
            println!("Entity attempting to equip item does not have a manipulator component");
            Self::failure()
        }
    }
}
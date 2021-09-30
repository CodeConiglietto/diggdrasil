use rand::prelude::*;
use specs::{world::EntitiesRes, Builder, Entity, LazyUpdate};

use crate::prelude::*;

pub enum EquipmentBuilder {
    Spear {
        head_material: Material,
        handle_material: Material,
    },
    Pick {
        head_material: Material,
        handle_material: Material,
    },
    Axe {
        head_material: Material,
        handle_material: Material,
    },
    Knife {
        blade_material: Material,
        handle_material: Material,
    },
}

impl EquipmentBuilder {
    pub fn build(&self, lazy: &LazyUpdate, entities: &EntitiesRes) -> Entity {
        match self {
            Self::Spear { head_material, .. } => lazy
                .create_entity(entities)
                .with(ItemComponent)
                .with(DrawComponent {
                    seed: thread_rng().gen::<usize>(),
                    sprite_builder: SpriteBuilder::Spear,
                    symbol_builder: Some(SymbolBuilder::Spear),
                })
                .with(AttackComponent {
                    available_attacks: vec![Attack {
                        name: String::from("thrust"),
                        attack_type: AttackType::Thrust,
                        attack_dice: DiceRoll::new_from_string("1d6"),
                    }],
                })
                .with(NameComponent {
                    name: format!("{} spear", head_material.get_name()),
                })
                .build(),
            Self::Pick { head_material, .. } => lazy
                .create_entity(entities)
                .with(ItemComponent)
                .with(DrawComponent {
                    seed: thread_rng().gen::<usize>(),
                    sprite_builder: SpriteBuilder::Pick,
                    symbol_builder: Some(SymbolBuilder::Pick),
                })
                .with(AttackComponent {
                    available_attacks: vec![Attack {
                        name: String::from("swing"),
                        attack_type: AttackType::Swing,
                        attack_dice: DiceRoll::new_from_string("1d4"),
                    }],
                })
                .with(NameComponent {
                    name: format!("{} pick", head_material.get_name()),
                })
                .build(),
                Self::Axe { head_material, .. } => lazy
                    .create_entity(entities)
                    .with(ItemComponent)
                    .with(DrawComponent {
                        seed: thread_rng().gen::<usize>(),
                        sprite_builder: SpriteBuilder::Axe,
                        symbol_builder: Some(SymbolBuilder::Axe),
                    })
                    .with(AttackComponent {
                        available_attacks: vec![Attack {
                            name: String::from("swing"),
                            attack_type: AttackType::Swing,
                            attack_dice: DiceRoll::new_from_string("1d6"),
                        }],
                    })
                    .with(NameComponent {
                        name: format!("{} axe", head_material.get_name()),
                    })
                    .build(),
                Self::Knife { blade_material, .. } => lazy
                    .create_entity(entities)
                    .with(ItemComponent)
                    .with(DrawComponent {
                        seed: thread_rng().gen::<usize>(),
                        sprite_builder: SpriteBuilder::Knife,
                        symbol_builder: Some(SymbolBuilder::Knife),
                    })
                    .with(AttackComponent {
                        available_attacks: vec![Attack {
                            name: String::from("stab"),
                            attack_type: AttackType::Stab,
                            attack_dice: DiceRoll::new_from_string("1d4"),
                        }],
                    })
                    .with(NameComponent {
                        name: format!("{} knife", blade_material.get_name()),
                    })
                    .build(),
        }
    }
}

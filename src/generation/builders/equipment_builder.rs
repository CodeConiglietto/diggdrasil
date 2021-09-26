use specs::{Builder, Entity, World as ECSWorld, WorldExt as ECSWorldExt};

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
}

impl EquipmentBuilder {
    pub fn build(&self, ecs_world: &mut ECSWorld) -> Entity {
        let builder = match self {
            Self::Spear {
                head_material,
                ..
            } => 
                ecs_world.create_entity()
                    .with(ItemComponent)
                    .with(NameComponent {name: format!("{} spear", head_material.get_name())}),
            Self::Pick {
                head_material,
                ..
            } => 
                ecs_world.create_entity()
                    .with(ItemComponent)
                    .with(NameComponent {name: format!("{} pick", head_material.get_name())}),
            Self::Axe {
                head_material,
                ..
            } => 
                ecs_world.create_entity()
                    .with(ItemComponent)
                    .with(NameComponent {name: format!("{} axe", head_material.get_name())}),
        };

        builder.build()
    }
}

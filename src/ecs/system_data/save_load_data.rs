use std::fmt::Display;

use serde::{Deserialize, Serialize};
use specs::{
    prelude::*,
    saveload::{ConvertSaveload, DeserializeComponents, SerializeComponents},
    storage::{GenericReadStorage, GenericWriteStorage},
};

use crate::prelude::*;

// Ignore the gross macros, move down
macro_rules! data_impl {
    (
        #[$meta:meta]
        pub struct SaveLoadData<'a> {
            $( $vis:vis $field:ident : $type:ty, )*
        }
    ) => {
        #[derive(Serialize, Deserialize)]
        pub struct SaveLoadDataComponents<'a> {
            $($field: Option<<<$type as GenericReadStorage>::Component as ConvertSaveload<SaveMarkerComponent>>::Data>,)*
        }
    }
}

macro_rules! ser_impl {
    (
        #[$meta:meta]
        pub struct SaveLoadData<'a> {
            $( $vis:vis $field:ident : $type:ty, )*
        }
    ) => {
        impl<'a, E> SerializeComponents<E, SaveMarkerComponent> for SaveLoadData<'a>
        where
            E: Display,
            $(
                E: From<<<$type as GenericReadStorage>::Component as ConvertSaveload<SaveMarkerComponent>>::Error>,
            )*
        {
            type Data = SaveLoadDataComponents<'a>;

            fn serialize_entity<F>(&self, entity: Entity, mut ids: F) -> Result<Self::Data, E>
            where
                F: FnMut(Entity) -> Option<SaveMarkerComponent>
            {
                Ok(
                    SaveLoadDataComponents {
                        $(
                            $field: self.$field.get(entity).map(|c| c.convert_into(&mut ids).map(Some)).unwrap_or(Ok(None))?,
                        )*
                    }
                )
            }
        }
    };
}

macro_rules! de_impl {
    (
        #[$meta:meta]
        pub struct SaveLoadData<'a> {
            $( $vis:vis $field:ident : $type:ty,)*
        }
    ) => {
        impl<'a, E> DeserializeComponents<E, SaveMarkerComponent> for SaveLoadData<'a> where
            E: Display,
            $(
                E: From<<<$type as GenericWriteStorage>::Component as ConvertSaveload<SaveMarkerComponent>>::Error>,
            )*
        {
            type Data = SaveLoadDataComponents<'a>;

            fn deserialize_entity<F>(
                &mut self,
                entity: Entity,
                components: Self::Data,
                mut ids: F,
            ) -> Result<(), E>
            where
                F: FnMut(SaveMarkerComponent) -> Option<Entity>
            {
                $(
                    if let Some(component) = components.$field {
                        self.$field.insert(entity, ConvertSaveload::<SaveMarkerComponent>::convert_from(component, &mut ids)?).unwrap();
                    } else {
                        self.$field.remove(entity);
                    }
                )*

                Ok(())
            }
        }
    };
}

macro_rules! save_load {
    ($($tt:tt)*) => {
        $($tt)*
            data_impl!{$($tt)*}
            ser_impl!{$($tt)*}
            de_impl!{$($tt)*}
    };
}

// Here starts the important stuff

save_load! {
    #[derive(SystemData)]
    pub struct SaveLoadData<'a> {
        pub ai_action: WriteStorage<'a, AIActionComponent>,
        pub ai_goal: WriteStorage<'a, AIGoalComponent>,
        pub attack: WriteStorage<'a, AttackComponent>,
        pub butcherable: WriteStorage<'a, ButcherableComponent>,
        pub collider: WriteStorage<'a, ColliderComponent>,
        pub collision: WriteStorage<'a, CollisionComponent>,
        pub death: WriteStorage<'a, DeathComponent>,
        pub digesion: WriteStorage<'a, DigestionComponent>,
        pub draw: WriteStorage<'a, DrawComponent>,
        pub edible: WriteStorage<'a, EdibleComponent>,
        pub health: WriteStorage<'a, HealthComponent>,
        pub intended_movement: WriteStorage<'a, IntendedMovementComponent>,
        pub inventory: WriteStorage<'a, InventoryComponent>,
        pub item: WriteStorage<'a, ItemComponent>,
        pub id: WriteStorage<'a, IdComponent>,
        pub manipulator: WriteStorage<'a, ManipulatorComponent>,
        pub material: WriteStorage<'a, MaterialComponent>,
        pub name: WriteStorage<'a, NameComponent>,
        pub particle_emitter: WriteStorage<'a, ParticleEmitterComponent>,
        pub position: WriteStorage<'a, PositionComponent>,
        pub velocity: WriteStorage<'a, VelocityComponent>,
    }
}

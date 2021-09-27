use serde::{Deserialize, Serialize};
use specs::{saveload::Marker, Component, VecStorage};

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct SaveMarkerComponent {
    pub id: u64,
}

impl Marker for SaveMarkerComponent {
    type Allocator = SaveMarkerAllocatorResource;
    type Identifier = u64;

    fn id(&self) -> Self::Identifier {
        self.id
    }
}

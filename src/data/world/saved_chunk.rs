use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct SavedChunk<'a> {
    pub chunk: Cow<'a, Chunk>,
    pub ids: Cow<'a, [u64]>,
}

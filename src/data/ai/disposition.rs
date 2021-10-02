use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub enum Disposition {
    Timid, 
    Neutral,
    Agressive
}
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Disposition {
    Timid, 
    Neutral,
    Agressive
}
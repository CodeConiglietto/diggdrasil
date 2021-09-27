use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WallFeature {
    Doorway,
    Window,
}

impl WallFeature {
    pub fn get_name(&self) -> String {
        match self {
            WallFeature::Doorway => String::from("doorway"),
            WallFeature::Window => String::from("window"),
        }
    }
}

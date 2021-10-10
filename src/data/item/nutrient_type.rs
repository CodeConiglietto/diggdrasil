use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum NutrientType {
    Plant,         //Plants
    Fungi,         //Fungi
    Meat,          //Meat
    AnimalProduct, //Anything that comes from an animal that isn't meat
}

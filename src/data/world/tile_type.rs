use strum::IntoEnumIterator;

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Ground,
    Wall {
        material: Material,
    },
    ConstructedWall {
        material: Material,
        material_shape: MaterialShape,
        wall_feature: Option<WallFeature>,
    },
}

impl TileType {
    pub fn collides(&self) -> bool {
        match self {
            TileType::Ground => false,
            TileType::Wall { .. } => true,
            TileType::ConstructedWall { wall_feature, .. } => {
                if let Some(wall_feature) = wall_feature {
                    match wall_feature {
                        WallFeature::Doorway => false,
                        _ => true,
                    }
                } else {
                    true
                }
            }
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            TileType::Ground => String::from("ground"),
            TileType::Wall { material } => format!("{} wall", material.get_name()),
            TileType::ConstructedWall {
                material,
                material_shape,
                wall_feature,
            } => format!(
                "{} {} {}",
                material.get_name(),
                material_shape.get_name(),
                if let Some(wall_feature) = wall_feature {
                    wall_feature.get_name()
                } else {
                    String::from("wall")
                }
            ),
        }
    }

    pub fn get_materials(&self) -> (Material, usize) {
        match self {
            TileType::Ground => (Material::Dirt, 10),
            TileType::Wall { material } => (*material, 10),
            TileType::ConstructedWall { material, .. } => (*material, 5),
        }
    }

    pub fn available_buildings(&self) -> Vec<Self> {
        match self {
            //TODO: get all possible material, shape and wall feature combinations for constructing a contructed wall
            TileType::Ground => Material::iter()
                .map(|material| TileType::ConstructedWall {
                    material,
                    material_shape: MaterialShape::Log,
                    wall_feature: None,
                })
                .collect(),
            TileType::Wall { .. } => Vec::new(),
            TileType::ConstructedWall { .. } => Vec::new(),
        }
    }

    pub fn connects(&self) -> bool {
        match self {
            TileType::Ground => false,
            TileType::Wall { .. } => true,
            TileType::ConstructedWall { .. } => true,
        }
    }
}

impl Default for TileType {
    fn default() -> TileType {
        unreachable!();
    }
}

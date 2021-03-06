use std::{env, path::PathBuf};

use ggez::event::{KeyCode, KeyMods};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

//TODO: find a good way to do this better
pub fn get_random_transforms_from_seed(seed: usize) -> (Rotation, Mirror) {
    (
        match (seed / 4) % 4 {
            0 => Rotation::None,
            1 => Rotation::Rotation90,
            2 => Rotation::Rotation180,
            3 => Rotation::Rotation270,
            _ => unreachable!(),
        },
        match ((seed + 100) / 4) % 4 {
            0 => Mirror::None,
            1 => Mirror::MirrorX,
            2 => Mirror::MirrorY,
            3 => Mirror::MirrorBoth,
            _ => unreachable!(),
        },
    )
}

pub fn fulfills_material_requirements(
    material: &MaterialComponent,
    (required_material, required_shape): (Option<Material>, Option<MaterialShape>),
) -> bool {
    if let Some(required_material) = required_material {
        if material.material != required_material {
            return false;
        }
    }

    if let Some(required_shape) = required_shape {
        if material.shape != required_shape {
            return false;
        }
    }

    true
}

pub fn key_to_index(keycode: KeyCode, keymods: KeyMods) -> Option<usize> {
    let mut index = match keycode {
        KeyCode::A => 0,
        KeyCode::B => 1,
        KeyCode::C => 2,
        KeyCode::D => 3,
        KeyCode::E => 4,
        KeyCode::F => 5,
        KeyCode::G => 6,
        KeyCode::H => 7,
        KeyCode::I => 8,
        KeyCode::J => 9,
        KeyCode::K => 10,
        KeyCode::L => 11,
        KeyCode::M => 12,
        KeyCode::N => 13,
        KeyCode::O => 14,
        KeyCode::P => 15,
        KeyCode::Q => 16,
        KeyCode::R => 17,
        KeyCode::S => 18,
        KeyCode::T => 19,
        KeyCode::U => 20,
        KeyCode::V => 21,
        KeyCode::W => 22,
        KeyCode::X => 23,
        KeyCode::Y => 24,
        KeyCode::Z => 25,

        _ => return None,
    };

    if keymods.contains(KeyMods::SHIFT) {
        index += 26
    }

    Some(index)
}

pub fn index_to_letter(index: usize) -> Option<char> {
    let c = match index {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        8 => 'i',
        9 => 'j',
        10 => 'k',
        11 => 'l',
        12 => 'm',
        13 => 'n',
        14 => 'o',
        15 => 'p',
        16 => 'q',
        17 => 'r',
        18 => 's',
        19 => 't',
        20 => 'u',
        21 => 'v',
        22 => 'w',
        23 => 'x',
        24 => 'y',
        25 => 'z',

        26 => 'A',
        27 => 'B',
        28 => 'C',
        29 => 'D',
        30 => 'E',
        31 => 'F',
        32 => 'G',
        33 => 'H',
        34 => 'I',
        35 => 'J',
        36 => 'K',
        37 => 'L',
        38 => 'M',
        39 => 'N',
        40 => 'O',
        41 => 'P',
        42 => 'Q',
        43 => 'R',
        44 => 'S',
        45 => 'T',
        46 => 'U',
        47 => 'V',
        48 => 'W',
        49 => 'X',
        50 => 'Y',
        51 => 'Z',

        _ => return None,
    };

    Some(c)
}

pub fn save_path() -> PathBuf {
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir)
    } else {
        env::current_dir().unwrap()
    }
    .join("saves")
}

pub fn serialize_data<T: Serialize>(t: &T, buf: &mut Vec<u8>) {
    rmp_serde::encode::write(buf, t).unwrap()
}

pub fn deserialize_data<'de, T: Deserialize<'de>>(buf: &'de [u8]) -> T {
    rmp_serde::decode::from_read_ref(buf).unwrap()
}

pub fn serializer(buf: &mut Vec<u8>) -> rmp_serde::encode::Serializer<&mut Vec<u8>> {
    rmp_serde::encode::Serializer::new(buf)
}

pub fn deserializer(
    buf: &[u8],
) -> rmp_serde::decode::Deserializer<rmp_serde::decode::ReadReader<&[u8]>> {
    rmp_serde::decode::Deserializer::new(buf)
}

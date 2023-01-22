use std::fs::File;

use bevy::utils::HashMap;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, Clone)]
pub struct RawBlockDataUnit {
    pub name: String,
    pub loc: [u32;6],
    pub durability: f32,
    pub multiplier: HashMap<String, f32>
}

#[derive(Serialize, Deserialize)]
pub struct RawBlockData {
    pub grid: u32,
    pub blocks: HashMap<u16, RawBlockDataUnit>,
}

lazy_static! {
    pub static ref RAW_BLOCK_DATA: RawBlockData = load_block_data();
    pub static ref BLOCK_DATA: HashMap<u16, RawBlockDataUnit> = {
        RAW_BLOCK_DATA.blocks.clone()
    };
    pub static ref TEXTURE_MAP_GRID_SIZE: u32 = RAW_BLOCK_DATA.grid;
}

pub fn load_block_data() -> RawBlockData {
    let blocks = match File::open("assets/blocks.json") {
        Ok(blocks) => blocks,
        Err(e) => {
            panic!("Error opening blocks.json: {}", e);
        }
    };

    match serde_json::from_reader(blocks) {
        Ok(v) => v,
        Err(e) => {
            panic!("Error parsing blocks.json: {}",e);
        }
    }
}

/// Returns the durability of the block or -1.0 if missing
#[inline]
pub fn get_durability(id: u16) -> f32 {
    BLOCK_DATA.get(&id).and_then(|unit| Some(unit.durability)).unwrap_or(-1.0)
}

/// Returns the requested multiplier or 1.0 if missing
#[inline]
pub fn get_multiplier(id: u16, name: &str) -> f32 {
    *BLOCK_DATA.get(&id).and_then(|unit| unit.multiplier.get(name)).unwrap_or(&1.0)
}
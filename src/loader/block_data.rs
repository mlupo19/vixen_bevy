use std::fs::File;

use bevy::{prelude::Commands, utils::HashMap};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BlockDataUnit {
    name: String,
    pub loc: [u32;6],
    durability: f32,
    multiplier: HashMap<String, f32>
}

#[derive(Serialize, Deserialize)]
pub struct RawBlockData {
    pub grid: u32,
    pub blocks: HashMap<u16, BlockDataUnit>,
}

pub fn load_block_data(mut commands: Commands) {
    let blocks = match File::open("assets/blocks.json") {
        Ok(blocks) => blocks,
        Err(e) => {
            panic!("Error opening blocks.json: {}", e);
        }
    };

    let data: RawBlockData = match serde_json::from_reader(blocks) {
        Ok(v) => v,
        Err(e) => {
            panic!("Error parsing blocks.json: {}",e);
        }
    };

    commands.insert_resource(data);
}
use bevy::math::ivec3;
use vixen_core::{terrain::{Structure, set_block_in_neighborhood}, Block};

use crate::StandardBlocks;

pub struct BrownMushroom;

const BROWN_MUSHROOM_HEIGHT: i32 = 6;
const BROWN_MUSHROOM_RADIUS: i32 = 3;

impl Structure for BrownMushroom {
    fn get_chance(&self) -> f64 {
        0.002
    }

    fn generate(&self, position: vixen_core::BlockCoord, in_progress: std::sync::Arc<dashmap::DashMap<vixen_core::ChunkCoord, vixen_core::loader::UnfinishedChunkData>>, _rng: &mut rand::rngs::StdRng) {
        let (x, y, z) = (position.x, position.y, position.z);
        
        // Build the stem
        for i in 0..BROWN_MUSHROOM_HEIGHT {
            set_block_in_neighborhood(ivec3(x,y + i, z), Block::from(StandardBlocks::MushroomStem), in_progress.clone());
        }

        // Build the cap
        let block = Block::from(StandardBlocks::BrownMushroom);
        for j in 0..BROWN_MUSHROOM_RADIUS {
            for k in 0..BROWN_MUSHROOM_RADIUS {
                if j == BROWN_MUSHROOM_RADIUS - 1 && k == BROWN_MUSHROOM_RADIUS - 1 {
                    continue;
                }
                set_block_in_neighborhood(ivec3(x + j, y + BROWN_MUSHROOM_HEIGHT, z + k), block, in_progress.clone());
                set_block_in_neighborhood(ivec3(x - j, y + BROWN_MUSHROOM_HEIGHT, z + k), block, in_progress.clone());
                set_block_in_neighborhood(ivec3(x + j, y + BROWN_MUSHROOM_HEIGHT, z - k), block, in_progress.clone());
                set_block_in_neighborhood(ivec3(x - j, y + BROWN_MUSHROOM_HEIGHT, z - k), block, in_progress.clone());
            }
        }
    }
}
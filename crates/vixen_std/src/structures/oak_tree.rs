use std::sync::Arc;

use bevy::math::ivec3;
use dashmap::DashMap;
use rand::Rng;
use vixen_core::loader::UnfinishedChunkData;
use vixen_core::terrain::{Structure, set_block_in_neighborhood};
use vixen_core::*;

use crate::StandardBlocks;

const OAK_CHANCE: f64 = 0.005;
const MIN_OAK_TREE_HEIGHT: usize = 4;
const MAX_OAK_TREE_HEIGHT: usize = 8;
const OAK_LEAVES_RADIUS: i32 = 3;
const OAK_LEAVES_HEIGHT: i32 = 4;

pub struct OakTree;

impl Structure for OakTree {
    fn get_chance(&self) -> f64 {
        OAK_CHANCE
    }

    fn generate(&self, position: BlockCoord, in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>, rng: &mut rand::rngs::StdRng) {
        let trunk_height = rng.gen_range(MIN_OAK_TREE_HEIGHT..=MAX_OAK_TREE_HEIGHT);

        // Build the trunk
        let block = Block::from(StandardBlocks::OakLog);
        for y in 0..trunk_height {
            set_block_in_neighborhood(position + ivec3(0, y as i32, 0), block, in_progress.clone());
        }

        // Build the leaves
        let block = Block::from(StandardBlocks::OakLeaves);
        for y in 1..OAK_LEAVES_HEIGHT-1 {
            for x in 0..OAK_LEAVES_RADIUS {
                for z in 0..OAK_LEAVES_RADIUS {
                    if x == OAK_LEAVES_RADIUS - 1 && z == OAK_LEAVES_RADIUS - 1 {
                        continue;
                    }
                    set_block_in_neighborhood(position + ivec3(x, trunk_height as i32 + y, z), block, in_progress.clone());
                    set_block_in_neighborhood(position + ivec3(-x, trunk_height as i32 + y, z), block, in_progress.clone());
                    set_block_in_neighborhood(position + ivec3(x, trunk_height as i32 + y, -z), block, in_progress.clone());
                    set_block_in_neighborhood(position + ivec3(-x, trunk_height as i32 + y, -z), block, in_progress.clone());
                }
            }
        }

        for x in 0..OAK_LEAVES_RADIUS-1 {
            for z in 0..OAK_LEAVES_RADIUS-1 {
                if x == OAK_LEAVES_RADIUS - 1 && z == OAK_LEAVES_RADIUS - 1 {
                    continue;
                }
                set_block_in_neighborhood(position + ivec3(x, trunk_height as i32, z), block, in_progress.clone());
                set_block_in_neighborhood(position + ivec3(-x, trunk_height as i32, z), block, in_progress.clone());
                set_block_in_neighborhood(position + ivec3(x, trunk_height as i32, -z), block, in_progress.clone());
                set_block_in_neighborhood(position + ivec3(-x, trunk_height as i32, -z), block, in_progress.clone());

                set_block_in_neighborhood(position + ivec3(x, trunk_height as i32 + OAK_LEAVES_HEIGHT - 1, z), block, in_progress.clone());
                set_block_in_neighborhood(position + ivec3(-x, trunk_height as i32 + OAK_LEAVES_HEIGHT - 1, z), block, in_progress.clone());
                set_block_in_neighborhood(position + ivec3(x, trunk_height as i32 + OAK_LEAVES_HEIGHT - 1, -z), block, in_progress.clone());
                set_block_in_neighborhood(position + ivec3(-x, trunk_height as i32 + OAK_LEAVES_HEIGHT - 1, -z), block, in_progress.clone());
            }
        }
    }
}
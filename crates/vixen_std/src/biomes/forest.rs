use std::sync::Arc;

use dashmap::DashMap;
use rand::Rng;
use vixen_core::{terrain::{Biome, Structure}, ChunkCoord, loader::UnfinishedChunkData, BlockCoord};

use crate::structures::OakTree;

pub struct ForestBiome;

impl Biome for ForestBiome {
    fn get_name(&self) -> &'static str {
        "Forest"
    }

    fn generate_structures(&self, block_coord: &BlockCoord, in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>, rng: &mut rand::rngs::StdRng) {
        if rng.gen_range(0..100) < 5 {
            OakTree.generate(*block_coord, in_progress, rng);
        }
    }
}
use std::sync::Arc;

use dashmap::DashMap;
use rand::Rng;
use vixen_core::{terrain::{Biome, Structure}, ChunkCoord, loader::UnfinishedChunkData, BlockCoord};

use crate::structures::{OakTree, BrownMushroom};

pub struct ForestBiome;

impl Biome for ForestBiome {
    fn get_name(&self) -> &'static str {
        "Forest"
    }

    fn generate_structures(&self, block_coord: &BlockCoord, in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>, rng: &mut rand::rngs::StdRng) {
        let chance = rng.gen::<f64>();
        
        if chance < OakTree.get_chance() {
            OakTree.generate(*block_coord, in_progress, rng);
        } else if chance - OakTree.get_chance() < BrownMushroom.get_chance() {
            BrownMushroom.generate(*block_coord, in_progress, rng);
        }


    }
}
use std::{sync::Arc, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use dashmap::DashMap;
use noise::NoiseFn;
use rand::{Rng, SeedableRng};
use vixen_core::{terrain::{Biome, Structure, get_block_from_chunk, set_block_in_chunk}, ChunkCoord, loader::{UnfinishedChunkData, ChunkData, CHUNK_SIZE}, BlockCoord, Block, chunk_local_to_block_coord};

use crate::{structures::{OakTree, BrownMushroom}, StandardBlocks};

pub struct ForestBiome;

impl Biome for ForestBiome {
    fn get_name(&self) -> &'static str {
        "Forest"
    }

    #[inline]
    fn generate_structures(&self, block_coord: &BlockCoord, in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>, rng: &mut rand::rngs::StdRng) {
        let chance = rng.gen::<f64>();
        
        if chance < OakTree.get_chance() {
            OakTree.generate(*block_coord, in_progress, rng);
        } else if chance - OakTree.get_chance() < BrownMushroom.get_chance() {
            BrownMushroom.generate(*block_coord, in_progress, rng);
        }
    }

    #[inline]
    fn generate_chunk(&self, coord: ChunkCoord, chunk_data: &mut ChunkData, in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>, noise: &Box<dyn NoiseFn<f64, 3> + Send + Sync>, seed: u32) {
        let (x,y,z) = (coord.x, coord.y, coord.z);
        let mut heights = ndarray::Array2::<i32>::zeros((CHUNK_SIZE.0, CHUNK_SIZE.2));
        for i in 0..CHUNK_SIZE.0 {
            for j in 0..CHUNK_SIZE.2 {
                let freq = 0.01;
                let height = 75.0
                 * noise.get([(x * CHUNK_SIZE.0 as i32 + i as i32) as f64 / (CHUNK_SIZE.0 as f32 / freq) as f64, (z * CHUNK_SIZE.2 as i32 + j as i32) as f64 / (CHUNK_SIZE.2 as f32 / freq) as f64, 0.0]); 

                heights[(i, j)] = height as i32;
            }
        }

        for i in 0..CHUNK_SIZE.0 {
            for j in 0..CHUNK_SIZE.1 {
                for k in 0..CHUNK_SIZE.2 {
                    let block_coord = chunk_local_to_block_coord(&(i as i32, j as i32, k as i32), &coord);
                    if get_block_from_chunk(&chunk_data, (i, j, k)).unwrap_or(Block::air()) != Block::air() {
                        continue;
                    }
                    if heights[(i, k)] > (j as i32 + y * CHUNK_SIZE.1 as i32) {
                        let block = match j as i32 + y * CHUNK_SIZE.1 as i32 {
                            // Grass layer
                            height if heights[(i,k)] - height == 1 => StandardBlocks::Grass,
                            // Dirt layer
                            height if heights[(i,k)] - height < 5 => StandardBlocks::Dirt,
                            // Stone layer
                            _ => StandardBlocks::Stone,
                        };
                        
                        set_block_in_chunk(chunk_data, (i, j, k), block.into());
                    } else if heights[(i, k)] == (j as i32 + y * CHUNK_SIZE.1 as i32) {
                        let mut hasher = DefaultHasher::new();
                        (seed, coord, i, j, k).hash(&mut hasher);
                        let mut rand = rand::rngs::StdRng::seed_from_u64(hasher.finish());
                        self.generate_structures(&block_coord, in_progress.clone(), &mut rand);
                    }
                }
            }
        }
    }
}
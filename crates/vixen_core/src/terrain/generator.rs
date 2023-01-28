use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, sync::Arc};

use dashmap::DashMap;
use noise::NoiseFn;
use rand::SeedableRng;

use crate::{util::{block_to_chunk_coord, chunk_local_to_block_coord, block_to_chunk_local_coord, ChunkCoord, BlockCoord}, loader::get_biome};
use crate::loader::{Chunk, CHUNK_SIZE, Block, ChunkData, UnfinishedChunkData};

use super::simple_noise::simple_noise;


trait ClonableNoiseFn: NoiseFn<f64, 3> + Send + Clone {}

pub struct TerrainGenerator {
    seed: u32,
    noise: Box<dyn NoiseFn<f64, 3> + Send + Sync>,
}

impl TerrainGenerator {
    /// Create a new Terrain Generator with a non-negative seed
    pub fn new(seed: u32) -> TerrainGenerator {
        let noise = simple_noise(seed);
        TerrainGenerator {
            seed,
            noise: Box::new(noise),
        }
    }

    pub fn generate_chunk(&self, loaded: u32, coord: ChunkCoord, in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>) -> (ChunkCoord, Chunk) {
        let mut c = 0;
        for x in (coord.x-1)..=(coord.x+1) {
            for y in (coord.y-1)..=(coord.y+1) {
                for z in (coord.z-1)..=(coord.z+1) {
                    let coord = ChunkCoord::new(x, y, z);
                    let in_progress = in_progress.clone();
                    if (loaded >> c) & 1 == 0
                        && match in_progress.get(&coord) {
                            Some(v) => !v.started,
                            _ => true
                        }
                    {
                        let generator = &self;
                        generator.gen(coord, in_progress);
                    }

                    c += 1;
                }
            }
        }

        let mut all_done;
        loop {
            all_done = true;
            for x in (coord.x-1)..=(coord.x+1) {
                for y in (coord.y-1)..=(coord.y+1) {
                    for z in (coord.z-1)..=(coord.z+1) {
                        let coord = ChunkCoord::new(x, y, z);
                        let in_progress = in_progress.clone();
                        if let Some(v) = in_progress.get(&coord) {
                            if !v.finished {
                                all_done = false;
                            }
                        };
                    }
                }
            }

            if all_done {
                break;
            }
        }

        let (_, UnfinishedChunkData { data: mut chunk_data, block_list: overlapped_blocks, started: _, finished: built }) = in_progress.remove(&coord).expect("Chunk should exist in list at this point");
        assert!(built, "Chunk should be built at this point");
        
        overlapped_blocks.into_iter().for_each(|((x,y,z), block)| {
            if chunk_data.is_none() {
                chunk_data = Some(Box::new(ndarray::Array3::default(CHUNK_SIZE)));
            }
            chunk_data.as_mut().unwrap()[(x,y,z)] = block;
        });

        let chunk = match chunk_data {
            Some(chunk_data) => Chunk::from_data(coord, chunk_data),
            None => Chunk::empty(coord),
        };

        (coord, chunk)
    }

    /// Generate chunk at coord (x,y,z) in chunk space
    fn gen(&self, coord: ChunkCoord, in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>) {
        let mut chunk_data = None;
        let (x,y,z) = (coord.x, coord.y, coord.z);
        
        let mut entry = in_progress.entry(coord).or_insert(UnfinishedChunkData {data: None, block_list: Vec::new(), started: true, finished: false});
        entry.started = true;
        
        if y > 4 || y < -4 {
            entry.finished = true;
            return;
        }
        drop(entry);

        let mut heights = ndarray::Array2::<i32>::zeros((CHUNK_SIZE.0, CHUNK_SIZE.2));
        for i in 0..CHUNK_SIZE.0 {
            for j in 0..CHUNK_SIZE.2 {
                let freq = 0.01;
                let height = 75.0
                 * self.noise.get([(x * CHUNK_SIZE.0 as i32 + i as i32) as f64 / (CHUNK_SIZE.0 as f32 / freq) as f64, (z * CHUNK_SIZE.2 as i32 + j as i32) as f64 / (CHUNK_SIZE.2 as f32 / freq) as f64, 0.0]); 

                heights[(i, j)] = height as i32;
            }
        }

        for i in 0..CHUNK_SIZE.0 {
            for j in 0..CHUNK_SIZE.1 {
                for k in 0..CHUNK_SIZE.2 {
                    let block_coord = chunk_local_to_block_coord(&(i as i32, j as i32, k as i32), &coord);
                    if get_block(&chunk_data, (i, j, k)).unwrap_or(Block::air()) != Block::air() {
                        continue;
                    }
                    if heights[(i, k)] > (j as i32 + y * CHUNK_SIZE.1 as i32) {
                        let id = match j as i32 + y * CHUNK_SIZE.1 as i32 {
                            // Grass layer
                            height if heights[(i,k)] - height == 1 => 1,
                            // Dirt layer
                            height if heights[(i,k)] - height < 5 => 2,
                            // Stone layer
                            _ => 3
                        };
                        set_block(&mut chunk_data, (i, j, k), Block::new(id));
                    } else if heights[(i, k)] == (j as i32 + y * CHUNK_SIZE.1 as i32) {
                        let mut hasher = DefaultHasher::new();
                        (self.seed, coord, i, j, k).hash(&mut hasher);
                        let mut rand = rand::rngs::StdRng::seed_from_u64(hasher.finish());
                        get_biome(0).unwrap().generate_structures(&block_coord, in_progress.clone(), &mut rand);
                    }
                }
            }
        }

        let mut entry = in_progress.entry(coord).or_insert(UnfinishedChunkData {data: None, block_list: Vec::new(), started: true, finished: false});
        entry.data = chunk_data;
        entry.finished = true;
    }

    /// Returns world seed
    pub fn get_seed(&self) -> u32 {
        self.seed
    }
}

fn get_block(chunk_data: &ChunkData, coord: (usize, usize, usize)) -> Option<Block> {
    let Some(chunk_data) = chunk_data else {
        return None;
    };

    Some(chunk_data[(coord.0, coord.1, coord.2)].clone())
}

fn set_block(chunk_data: &mut ChunkData, coord: (usize, usize, usize), block: Block) {
    if let Some(chunk_data) = chunk_data {
        chunk_data[(coord.0, coord.1, coord.2)] = block;
    } else {
        *chunk_data = Some(Box::new(ndarray::Array3::default(CHUNK_SIZE)));
        chunk_data.as_deref_mut().unwrap()[(coord.0, coord.1, coord.2)] = block;
    }
}

pub fn set_block_in_neighborhood(coord: BlockCoord, block: Block, in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>) {
    let chunk_coord = block_to_chunk_coord(&coord);
    let local_coord = block_to_chunk_local_coord(&coord);

    let mut entry = in_progress.entry(chunk_coord).or_insert(UnfinishedChunkData {data: None, block_list: Vec::new(), started: false, finished: false});
    entry.block_list.push((local_coord, block));
}

#[cfg(test)]
mod tests {
    use bevy::math::ivec3;

    use super::*;

    #[test]
    fn test_chunk_generation_perf() {
        let generator = TerrainGenerator::new(0);
        let in_progress = Arc::new(DashMap::new());
        
        // Start timing
        let start = std::time::Instant::now();
        
        for x in -5..=5 {
            for y in -5..=5 {
                for z in -5..=5 {
                    let coord = ivec3(x, y, z);
                    let _ = generator.generate_chunk(0, coord, in_progress.clone());
                }
            }
        }

        // End timing
        let end = std::time::Instant::now();
        println!("Time to generate chunks is: {:?}", end - start);
    }
}
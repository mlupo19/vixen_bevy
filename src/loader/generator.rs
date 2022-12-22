use std::{sync::Arc, hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use bevy::{math::{IVec3, ivec3}, utils::HashMap, prelude::info};
use noise::{Perlin, Seedable, NoiseFn};
use rand::{SeedableRng, RngCore, Rng};

use crate::util::{block_to_chunk_coord, chunk_local_to_block_coord, block_to_chunk_local_coords};

use super::{chunk::{Chunk, CHUNK_SIZE, Block}, block_data::get_durability, ChunkCoord};

#[derive(Clone)]
pub struct TerrainGenerator {
    seed: u32,
    noise: noise::Perlin,
    
}

impl TerrainGenerator {
    /// Create a new Terrain Generator with a non-negative seed
    pub fn new(seed: u32) -> TerrainGenerator {
        let noise = Perlin::new().set_seed(seed);
        TerrainGenerator {
            seed,
            noise,
        }
    }

    /// Generate chunk at coord (x,y,z) in chunk space
    pub fn gen(&self, coord: ChunkCoord) -> (ChunkCoord, Chunk) {
        let mut out = Chunk::empty(coord);
        let (x,y,z) = (coord.x, coord.y, coord.z);

        if y > 4 || y < -4 {
            return (coord, out);
        }

        let mut heights = ndarray::Array2::<i32>::zeros((CHUNK_SIZE.0, CHUNK_SIZE.2));
        for i in 0..CHUNK_SIZE.0 {
            for j in 0..CHUNK_SIZE.2 {
                let freq = 0.05;
                let octaves = 4;
                let height = 120.0
                    * self.acc_noise(
                        octaves,
                        (x * CHUNK_SIZE.0 as i32 + i as i32) as f32 / (CHUNK_SIZE.0 as f32 / freq),
                        (z * CHUNK_SIZE.2 as i32 + j as i32) as f32 / (CHUNK_SIZE.2 as f32 / freq),
                    );

                heights[(i, j)] = height as i32;
            }
        }

        for i in 0..CHUNK_SIZE.0 {
            for j in 0..CHUNK_SIZE.1 {
                for k in 0..CHUNK_SIZE.2 {
                    // let (world_x,world_y,world_z) = (x * CHUNK_SIZE.0 as i32 + i as i32, y * CHUNK_SIZE.1 as i32 + j as i32, z * CHUNK_SIZE.2 as i32 + k as i32);
                    if out.get_block((i, j, k)).unwrap_or(Block::air()) != Block::air() {
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
                        out.set_block((i, j, k), Block::new(id, get_durability(id)));
                    } else if heights[(i, k)] == (j as i32 + y * CHUNK_SIZE.1 as i32) {
                        let mut hasher = DefaultHasher::new();
                        self.seed.hash(&mut hasher);
                        coord.hash(&mut hasher);
                        (i,j,k).hash(&mut hasher);
                        let mut rand = rand::rngs::StdRng::seed_from_u64(hasher.finish());

                        // Generate tree (0.05% chance)
                        if rand.gen::<f64>() < 0.0005 {
                            for m in 0..5 {
                                Self::set_block(&mut out, (i as i32,j as i32 + m,k as i32), Block::new(6, get_durability(6)));
                            }
                            for dx in -1..=1 {
                                for dy in 0..2 {
                                    for dz in -1..=1 {
                                        Self::set_block(&mut out, (i as i32 + dx, j as i32 + 5 + dy, k as i32 + dz), Block::new(7, get_durability(7)));
                                    }
                                }
                            }
                            Self::set_block(&mut out, (i as i32, j as i32 + 7, k as i32), Block::new(7, get_durability(7)));
                        }
                    }
                }
            }
        }

        (coord, out)
    }

    fn set_block(chunk: &mut Chunk, coord: (i32, i32, i32), block: Block) {
        let block_coord = chunk_local_to_block_coord(&coord, &chunk.get_coord());
        let chunk_coord = block_to_chunk_coord(&block_coord);
        if chunk_coord == chunk.get_coord() {
            chunk.set_block((coord.0 as usize, coord.1 as usize, coord.2 as usize), block);
        } else {
            // info!("Block outside of chunk");
        }
    }

    /// Returns world seed
    pub fn get_seed(&self) -> u32 {
        self.seed
    }

    fn acc_noise(&self, octaves: i32, x: f32, y: f32) -> f32 {
        let mut x = x;
        let mut y = y;
        let mut result = 0.0;
        let mut amp = 1.0;

        for _ in 0..octaves {
            result += self.noise.get([x as f64, y as f64, 0.0]) * amp;
            x *= 2.0;
            y *= 2.0;
            amp /= 2.0;
        }

        result as f32
    }
}

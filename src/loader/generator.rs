use bevy::math::IVec3;
use noise::{Perlin, Seedable, NoiseFn};

use super::chunk::{Chunk, CHUNK_SIZE, Block};

#[derive(Clone)]
pub struct TerrainGenerator {
    seed: u32,
    noise: noise::Perlin,
    sea_level: i32,
    
}

impl TerrainGenerator {
    /// Create a new Terrain Generator with a non-negative seed
    pub fn new(seed: u32) -> TerrainGenerator {
        let noise = Perlin::new().set_seed(seed);
        TerrainGenerator {
            seed,
            noise,
            sea_level: 60,
        }
    }

    /// Generate chunk at coord (x,y,z) in chunk space
    pub fn gen(&self, coord: IVec3) -> (IVec3, Chunk) {
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
                    if heights[(i, k)] > (j as i32 + y * CHUNK_SIZE.1 as i32) {
                        let id = match j as i32 + y * CHUNK_SIZE.1 as i32 {
                            height if heights[(i,k)] - height == 1 => 1,
                            height if heights[(i,k)] - height < 5 => 2,
                            _ => 3
                        };
                        out.set_block((i, j, k), Block::new(id, 5.0));
                    }
                }
            }
        }

        (coord, out)
    }

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

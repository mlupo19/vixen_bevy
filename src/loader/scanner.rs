use bevy::{math::*, prelude::Component};

use super::{chunk::CHUNK_SIZE, ChunkCoord};

#[derive(Component, Clone, Copy)]
pub struct ChunkScanner {
    range: u32,
    center: ChunkCoord,
}

impl ChunkScanner {
    pub fn new(range: u32, center: ChunkCoord) -> Self {
        Self { range, center }
    }

    pub fn update(&mut self, pos: Vec3) {
        self.center = ivec3((pos.x / CHUNK_SIZE.0 as f32).floor() as i32, (pos.y / CHUNK_SIZE.1 as f32).floor() as i32, (pos.z / CHUNK_SIZE.2 as f32).floor() as i32);
    }

    pub fn should_unload_chunk(&self, pos: &ChunkCoord) -> bool {
        let out = (pos.x - self.center.x).pow(2) + (pos.y - self.center.y).pow(2) + (pos.z - self.center.z).pow(2) > self.range.pow(2).try_into().unwrap();
        println!("{out} {pos} {}", self.center);
        out
    }

    pub fn should_unload_mesh(&self, pos: &ChunkCoord) -> bool {
        (pos.x - self.center.x).pow(2) + (pos.y - self.center.y).pow(2) + (pos.z - self.center.z).pow(2) > (self.range - 1).pow(2).try_into().unwrap()
    }
}

impl IntoIterator for ChunkScanner {
    type Item = ChunkCoord;

    type IntoIter = std::vec::IntoIter<ChunkCoord>;

    fn into_iter(self) -> Self::IntoIter {
        let from = -(self.range as i32);
        let to = self.range as i32;
        (from..to).flat_map(move |x| (from..to).flat_map(move |y| (from..to).map(move |z| ivec3(x + self.center.x, y + self.center.y, z + self.center.z)))).collect::<Vec<_>>().into_iter()
    }
}

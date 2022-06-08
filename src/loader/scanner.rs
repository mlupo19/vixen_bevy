use bevy::{math::*, prelude::Component};

use crate::loader::{to_chunk_coord, to_world_coord};

use super::{chunk::CHUNK_SIZE, ChunkCoord};

#[derive(Component, Clone, Copy)]
pub struct ChunkScanner {
    range: u16,
    center: Vec3,
}

impl ChunkScanner {
    pub fn new(range: u16, center: ChunkCoord) -> Self {
        Self { range, center: to_world_coord(&center) }
    }

    pub fn update(&mut self, pos: Vec3) {
        self.center = pos;
    }

    pub fn should_unload_chunk(&self, pos: &ChunkCoord) -> bool {
        let center = to_chunk_coord(&self.center);
        center.x.abs_diff(pos.x) > self.range.into() || center.y.abs_diff(pos.y) > self.range.into() || center.z.abs_diff(pos.z) > self.range.into()
    }

    pub fn should_load_mesh(&self, pos: &ChunkCoord) -> bool {
        let pos = to_world_coord(pos);
        let lhs: f32 = (pos.x - self.center.x).powf(2.0) + (pos.y - self.center.y).powf(2.0) + (pos.z - self.center.z).powf(2.0);
        let rhs: f32 = ((self.range - 1) as usize * 32).pow(2) as f32;
        lhs <= rhs
    }

    pub fn get_center(&self) -> ChunkCoord {
        self.center.floor().as_ivec3()
    }
}

impl IntoIterator for ChunkScanner {
    type Item = ChunkCoord;

    type IntoIter = std::vec::IntoIter<ChunkCoord>;

    fn into_iter(self) -> Self::IntoIter {
        let from = -(self.range as i32);
        let to = self.range as i32;
        let center = to_chunk_coord(&self.center);
        (from..to).flat_map(move |x| (from..to).flat_map(move |y| (from..to).map(move |z| ivec3(x + center.x, y + center.y, z + center.z)))).collect::<Vec<_>>().into_iter()
    }
}

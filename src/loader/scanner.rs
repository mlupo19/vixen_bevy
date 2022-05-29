use bevy::{math::*, prelude::Component};

use super::chunk::CHUNK_SIZE;

#[derive(Component, Clone, Copy)]
pub struct ChunkScanner {
    range: u32,
    center: IVec3,
}

impl ChunkScanner {
    pub fn new(range: u32, center: IVec3) -> Self {
        Self { range, center }
    }

    pub fn update(&mut self, pos: Vec3) {
        self.center = ivec3((pos.x / CHUNK_SIZE.0 as f32) as i32, (pos.y / CHUNK_SIZE.1 as f32) as i32, (pos.z / CHUNK_SIZE.2 as f32) as i32);
    }
}

impl IntoIterator for ChunkScanner {
    type Item = IVec3;

    type IntoIter = std::vec::IntoIter<IVec3>;

    fn into_iter(self) -> Self::IntoIter {
        let from = -(self.range as i32);
        let to = self.range as i32;
        (from..to).flat_map(move |x| (from..to).flat_map(move |y| (from..to).map(move |z| ivec3(x + self.center.x, y + self.center.y, z + self.center.z)))).collect::<Vec<_>>().into_iter()
    }
}

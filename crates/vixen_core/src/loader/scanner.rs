use bevy::{math::*, prelude::Component};

use crate::util::{to_chunk_coord, to_world_coord};

use super::ChunkCoord;

#[derive(Component, Clone)]
pub struct ChunkScanner {
    range: u32,
    center: Vec3,
    list: Vec<ChunkCoord>,
}

impl ChunkScanner {
    pub fn new(range: u32, center: ChunkCoord) -> Self {
        Self {
            range,
            center: to_world_coord(&center),
            list: Vec::new(),
        }
    }

    pub fn update(&mut self, pos: Vec3) {
        self.center = pos;
    }

    pub fn should_unload_chunk(&self, pos: &ChunkCoord) -> bool {
        let center = to_chunk_coord(&self.center);
        center.x.abs_diff(pos.x) > self.range + 1
            || center.y.abs_diff(pos.y) > self.range + 1
            || center.z.abs_diff(pos.z) > self.range + 1
    }

    pub fn should_unload_unfinished_chunk(&self, pos: &ChunkCoord) -> bool {
        let center = to_chunk_coord(&self.center);
        center.x.abs_diff(pos.x) > self.range as u32 + 3
            || center.y.abs_diff(pos.y) > self.range as u32 + 3
            || center.z.abs_diff(pos.z) > self.range as u32 + 3
    }

    pub fn should_load_mesh(&self, pos: &ChunkCoord) -> bool {
        let pos = to_world_coord(pos);
        let lhs: f32 = (pos.x - self.center.x).powf(2.0)
            + (pos.y - self.center.y).powf(2.0)
            + (pos.z - self.center.z).powf(2.0);
        let rhs: f32 = (self.range as usize * 32).pow(2) as f32;
        lhs <= rhs
    }

    pub fn get_center(&self) -> ChunkCoord {
        self.center.floor().as_ivec3()
    }

    pub fn range(&self) -> u32 {
        self.range
    }
}

impl<'a> IntoIterator for &'a mut ChunkScanner {
    type Item = ChunkCoord;

    type IntoIter = std::vec::Drain<'a, ChunkCoord>;

    fn into_iter(self) -> Self::IntoIter {
        let from = -(self.range as i32);
        let to = self.range as i32;
        let center = to_chunk_coord(&self.center);
        self.list.extend((from..to).flat_map(move |x| {
            (from..to).flat_map(move |y| {
                (from..to).map(move |z| ivec3(x + center.x, y + center.y, z + center.z))
            })
        }));
        self.list.drain(..)
    }
}

use bevy::{math::ivec3, prelude::IVec3};
use crate::loader::CHUNK_SIZE;

#[inline]
pub fn block_to_chunk_coord(block_coord: &IVec3) -> IVec3 {
    ivec3(
        (block_coord.x as f32 / CHUNK_SIZE.0 as f32).floor() as i32,
        (block_coord.y as f32 / CHUNK_SIZE.1 as f32).floor() as i32,
        (block_coord.z as f32 / CHUNK_SIZE.2 as f32).floor() as i32,
    )
}

#[inline]
pub fn chunk_local_to_block_coord(local_chunk_coord: &(i32, i32, i32), chunk_coord: &IVec3) -> IVec3 {
    ivec3(
        local_chunk_coord.0 + chunk_coord.x * CHUNK_SIZE.0 as i32,
        local_chunk_coord.1 + chunk_coord.y * CHUNK_SIZE.1 as i32,
        local_chunk_coord.2 + chunk_coord.z * CHUNK_SIZE.2 as i32,
    )
}

#[inline]
pub fn block_to_chunk_local_coords(block_coord: &IVec3) -> (usize, usize, usize) {
    let chunk_coord = block_to_chunk_coord(block_coord);
    (
        (block_coord.x - chunk_coord.x * CHUNK_SIZE.0 as i32) as usize,
        (block_coord.y - chunk_coord.y * CHUNK_SIZE.1 as i32) as usize,
        (block_coord.z - chunk_coord.z * CHUNK_SIZE.2 as i32) as usize,
    )
}
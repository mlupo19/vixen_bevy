use bevy::utils::HashMap;
use ndarray::Array3;

use super::{MeshData, ChunkCoord};
pub const CHUNK_SIZE: (usize, usize, usize) = (32, 32, 32);

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Block {
    pub id: u16,
    pub health: f32,
}

impl Block {
    pub fn new(id: u16, health: f32) -> Block {
        Block {
            id,
            health,
        }
    }

    pub fn air() -> Block {
        Block { id: 0, health: 0.0 }
    }

    pub fn is_air(&self) -> bool {
        self.id == 0
    }
}

struct Faces;
struct Face {
    points: &'static [(i32, i32, i32); 4],
    normal: (i32, i32, i32),
    face_id: u8,
}

impl Faces {
    pub const RIGHT: &'static Face = &Face {
        points: &[(1, 0, 0), (1, 1, 0), (1, 1, 1), (1, 0, 1)],
        normal: (1, 0, 0),
        face_id: 0,
    };
    pub const LEFT: &'static Face = &Face {
        points: &[(0, 0, 1), (0, 1, 1), (0, 1, 0), (0, 0, 0)],
        normal: (-1, 0, 0),
        face_id: 1,
    };
    pub const BOTTOM: &'static Face = &Face {
        points: &[(1, 0, 0), (1, 0, 1), (0, 0, 1), (0, 0, 0)],
        normal: (0, -1, 0),
        face_id: 2,
    };
    pub const TOP: &'static Face = &Face {
        points: &[(1, 1, 1), (1, 1, 0), (0, 1, 0), (0, 1, 1)],
        normal: (0, 1, 0),
        face_id: 3,
    };
    pub const FRONT: &'static Face = &Face {
        points: &[(1, 0, 1), (1, 1, 1), (0, 1, 1), (0, 0, 1)],
        normal: (0, 0, 1),
        face_id: 4,
    };
    pub const BACK: &'static Face = &Face {
        points: &[(0, 0, 0), (0, 1, 0), (1, 1, 0), (1, 0, 0)],
        normal: (0, 0, -1),
        face_id: 5,
    };
}

pub struct Chunk {
    coord: ChunkCoord,
    block_data: Option<Box<ndarray::Array3<Block>>>,
    needs_update: bool,
}

impl Chunk {
    pub fn empty(coord: ChunkCoord) -> Chunk {
        Chunk {
            coord,
            block_data: None,
            needs_update: false,
        }
    }

    pub fn new(coord: ChunkCoord) -> Chunk {
        Chunk {
            coord,
            block_data: None,
            needs_update: true,
        }
    }

    pub fn from_data(coord: ChunkCoord, data: Array3<Block>) -> Chunk {
        Chunk {
            coord,
            block_data: Some(Box::new(data)),
            needs_update: true,
        }
    }

    fn add_face(
        block_data: &Array3<Block>,
        vertex_data: &mut VertexDataList,
        indices: &mut Vec<u32>,
        (i, j, k): (usize, usize, usize),
        face: &Face,
        texture_map_info: &HashMap<u16, [[[u32; 2]; 4]; 6]>,
    ) {
        const FACE_INDICES: &[i32; 6] = &[0, 1, 2, 2, 3, 0];
        let mut mesh_face_index_loc: [usize; 4] = [0; 4];

        for c in 0..4 {
            let (fx, fy, fz) = face.points.get(c).unwrap();
            let point_in_chunk_space = (i as i32 + fx, j as i32 + fy, k as i32 + fz);
            mesh_face_index_loc[c] = vertex_data.0.len() as usize;

            let face_tex_coords = texture_map_info
                .get(&block_data.get((i, j, k)).unwrap().id)
                .unwrap()[face.face_id as usize];

            // vertices_normals_uvs.push(
            //     (point_in_chunk_space.0 as u32)
            //     | (point_in_chunk_space.1 as u32) << 6
            //     | (point_in_chunk_space.2 as u32) << 12
            //     | (face.normal.0 as u32) << 18
            //     | (face.normal.1 as u32) << 19
            //     | (face.normal.2 as u32) << 20
            //     | face_tex_coords[c][0] << 21
            //     | face_tex_coords[c][1] << 22);
            vertex_data.0.push([point_in_chunk_space.0 as f32, point_in_chunk_space.1 as f32, point_in_chunk_space.2 as f32]);
            vertex_data.1.push([face.normal.0 as f32, face.normal.1 as f32, face.normal.2 as f32]);
            vertex_data.2.push([face_tex_coords[c][0] as f32 / 16.0, face_tex_coords[c][1] as f32 / 16.0]);
        }

        for ind in FACE_INDICES.iter() {
            indices.push(mesh_face_index_loc[*ind as usize] as u32);
        }
    }

    pub fn gen_mesh(
        block_data: &Array3<Block>,
        neighbors: [Option<&Box<Array3<Block>>>;6],
        texture_map_info: &HashMap<u16, [[[u32; 2]; 4]; 6]>,
    ) -> MeshData {
        let presize = CHUNK_SIZE.0 * CHUNK_SIZE.1 * CHUNK_SIZE.2;
        let mut vertex_data = VertexDataList(Vec::with_capacity(presize), Vec::with_capacity(presize), Vec::with_capacity(presize));
        let mut indices = Vec::with_capacity(CHUNK_SIZE.0 * CHUNK_SIZE.1 * CHUNK_SIZE.2 * 3);

        for i in 0..CHUNK_SIZE.0 {
            for j in 0..CHUNK_SIZE.1 {
                for k in 0..CHUNK_SIZE.2 {
                    // Check if block or air
                    if block_data.get((i, j, k)).unwrap().id != 0 {
                        // Check adjacent blocks

                        // Add right face to mesh
                        if i == CHUNK_SIZE.0 - 1
                            || block_data
                                .get((i + 1, j, k))
                                .unwrap()
                                .id
                                == 0
                        {
                            // Check neighbor chunk if block is on edge
                            if i == CHUNK_SIZE.0 - 1 {
                                let neighbor = &neighbors[0];
                                if neighbor.is_none()
                                    || neighbor
                                        .as_ref()
                                        .unwrap()
                                        .get((0, j, k))
                                        .unwrap_or(&Block { id: 0, health: 0.0 })
                                        .id
                                        == 0
                                {
                                    Chunk::add_face(
                                        &block_data,
                                        &mut vertex_data,
                                        &mut indices,
                                        (i, j, k),
                                        Faces::RIGHT,
                                        &texture_map_info,
                                    );
                                }
                            } else {
                                Chunk::add_face(
                                    &block_data,
                                    &mut vertex_data,
                                    &mut indices,
                                    (i, j, k),
                                    Faces::RIGHT,
                                    &texture_map_info,
                                );
                            }
                        }

                        // Add left face to mesh
                        if i == 0
                            || block_data
                                .get((i - 1, j, k))
                                .unwrap()
                                .id
                                == 0
                        {
                            if i == 0 {
                                let neighbor = &neighbors[1];
                                if neighbor.is_none()
                                    || neighbor
                                        .as_ref()
                                        .unwrap()
                                        .get((CHUNK_SIZE.0 - 1, j, k))
                                        .unwrap_or(&Block { id: 0, health: 0.0 })
                                        .id
                                        == 0
                                {
                                    Chunk::add_face(
                                        &block_data,
                                        &mut vertex_data,
                                        &mut indices,
                                        (i, j, k),
                                        Faces::LEFT,
                                        &texture_map_info,
                                    );
                                }
                            } else {
                                Chunk::add_face(
                                    &block_data,
                                    &mut vertex_data,
                                    &mut indices,
                                    (i, j, k),
                                    Faces::LEFT,
                                    &texture_map_info,
                                );
                            }
                        }

                        // Add bottom face to mesh
                        if j == 0
                            || block_data
                                .get((i, j - 1, k))
                                .unwrap()
                                .id
                                == 0
                        {
                            if j == 0 {
                                let neighbor = &neighbors[2];
                                if neighbor.is_none()
                                    || neighbor
                                        .as_ref()
                                        .unwrap()
                                        .get((i, CHUNK_SIZE.1 - 1, k))
                                        .unwrap_or(&Block { id: 0, health: 0.0 })
                                        .id
                                        == 0
                                {
                                    Chunk::add_face(
                                        &block_data,
                                        &mut vertex_data,
                                        &mut indices,
                                        (i, j, k),
                                        Faces::BOTTOM,
                                        &texture_map_info,
                                    );
                                }
                            } else {
                                Chunk::add_face(
                                    &block_data,
                                    &mut vertex_data,
                                    &mut indices,
                                    (i, j, k),
                                    Faces::BOTTOM,
                                    &texture_map_info,
                                );
                            }
                        }

                        // Add top face to mesh
                        if j == CHUNK_SIZE.1 - 1
                            || block_data
                                .get((i, j + 1, k))
                                .unwrap()
                                .id
                                == 0
                        {
                            if j == CHUNK_SIZE.1 - 1 {
                                let neighbor = &neighbors[3];
                                if neighbor.is_none()
                                    || neighbor
                                        .as_ref()
                                        .unwrap()
                                        .get((i, 0, k))
                                        .unwrap_or(&Block { id: 0, health: 0.0 })
                                        .id
                                        == 0
                                {
                                    Chunk::add_face(
                                        &block_data,
                                        &mut vertex_data,
                                        &mut indices,
                                        (i, j, k),
                                        Faces::TOP,
                                        &texture_map_info,
                                    );
                                }
                            } else {
                                Chunk::add_face(
                                    &block_data,
                                    &mut vertex_data,
                                    &mut indices,
                                    (i, j, k),
                                    Faces::TOP,
                                    &texture_map_info,
                                );
                            }
                        }

                        // Add front face to mesh
                        if k == CHUNK_SIZE.2 - 1
                            || block_data
                                .get((i, j, k + 1))
                                .unwrap()
                                .id
                                == 0
                        {
                            if k == CHUNK_SIZE.2 - 1 {
                                let neighbor = &neighbors[4];
                                if neighbor.is_none()
                                    || neighbor
                                        .as_ref()
                                        .unwrap()
                                        .get((i, j, 0))
                                        .unwrap_or(&Block { id: 0, health: 0.0 })
                                        .id
                                        == 0
                                {
                                    Chunk::add_face(
                                        &block_data,
                                        &mut vertex_data,
                                        &mut indices,
                                        (i, j, k),
                                        Faces::FRONT,
                                        &texture_map_info,
                                    );
                                }
                            } else {
                                Chunk::add_face(
                                    &block_data,
                                    &mut vertex_data,
                                    &mut indices,
                                    (i, j, k),
                                    Faces::FRONT,
                                    &texture_map_info,
                                );
                            }
                        }

                        // Add back face to mesh
                        if k == 0
                            || block_data
                                .get((i, j, k - 1))
                                .unwrap()
                                .id
                                == 0
                        {
                            if k == 0 {
                                let neighbor = &neighbors[5];
                                if neighbor.is_none()
                                    || neighbor
                                        .as_ref()
                                        .unwrap()
                                        .get((i, j, CHUNK_SIZE.2 - 1))
                                        .unwrap_or(&Block { id: 0, health: 0.0 })
                                        .id
                                        == 0
                                {
                                    Chunk::add_face(
                                        &block_data,
                                        &mut vertex_data,
                                        &mut indices,
                                        (i, j, k),
                                        Faces::BACK,
                                        &texture_map_info,
                                    );
                                }
                            } else {
                                Chunk::add_face(
                                    &block_data,
                                    &mut vertex_data,
                                    &mut indices,
                                    (i, j, k),
                                    Faces::BACK,
                                    &texture_map_info,
                                );
                            }
                        }
                    }
                }
            }
        }

        (vertex_data.0, vertex_data.1, vertex_data.2, indices)
    }

    pub fn set_block(&mut self, (i, j, k): (usize, usize, usize), block: Block) -> bool {
        let mut needs_update = false;

        match self.block_data {
            None => {
                self.block_data = Some(Box::new(ndarray::Array3::default(CHUNK_SIZE)));
                needs_update = true;
            },
            Some(_) => {
                if self.block_data.as_ref().unwrap()[[i, j, k]] != block {
                    self.needs_update = true;
                    needs_update = true;
                }
            }
        }

        self.block_data.as_mut().unwrap()[[i, j, k]] = block;

        needs_update
    }

    pub fn get_block(&self, (i, j, k): (usize, usize, usize)) -> Option<Block> {
        match &self.block_data {
            None => Some(Block { id: 0, health: 69.0 }),
            Some(data) => Some(data[(i, j, k)].clone()),
        }
    }

    // pub fn get_pos(&self) -> (i32, i32, i32) {
    //     self.position
    // }

    pub fn get_data_mut(&mut self) -> &mut Option<Box<ndarray::Array3<Block>>> {
        &mut self.block_data
    }

    pub fn get_data(&self) -> &Option<Box<ndarray::Array3<Block>>> {
        &self.block_data
    }

    pub fn set_updated(&mut self) {
        self.needs_update = false;
    }

    pub fn is_empty(&self) -> bool {
        self.block_data.is_none()
    }

    pub fn needs_update(&self) -> bool {
        self.needs_update
    }

    pub fn request_update(&mut self) {
        self.needs_update = true;
    }
}

struct VertexDataList(Vec<[f32;3]>, Vec<[f32;3]>, Vec<[f32;2]>);
use std::sync::Arc;

use bevy::{utils::HashMap, math::ivec3};
use dashmap::DashMap;
use ndarray::Array3;
use crate::{loader::*, terrain::TerrainGenerator, util::BlockCoord};

pub type ChunkMap = HashMap<ChunkCoord, Chunk>;

#[derive(Resource)]
pub struct Worldgen {
    chunk_map: ChunkMap,
    mesh_map: HashMap<ChunkCoord, Handle<Mesh>>,
    generator: Arc<TerrainGenerator>,
    needs_mesh_build: HashSet<ChunkCoord>,
    needs_chunk_build: HashSet<ChunkCoord>, 
    in_progress: Arc<DashMap<ChunkCoord, UnfinishedChunkData>>,
}

impl Worldgen {
    pub fn new(seed: u32) -> Self {
        Self {
            generator: Arc::new(TerrainGenerator::new(seed)),
            chunk_map: Default::default(),
            mesh_map: Default::default(),
            needs_mesh_build: Default::default(),
            needs_chunk_build: Default::default(),
            in_progress: Arc::new(DashMap::new()),
        }
    }

    pub fn scan_chunks(&mut self, mut scanner: Query<&mut ChunkScanner>, mut commands: Commands) {
        let pool = AsyncComputeTaskPool::get();
        for mut scanner in scanner.iter_mut() {
            for chunk_coord in scanner.into_iter() {
                if !self.chunk_map.contains_key(&chunk_coord) && !self.needs_chunk_build.contains(&chunk_coord) {
                    self.needs_chunk_build.insert(chunk_coord);
                    let generator = self.generator.clone();
                    let in_progress = self.in_progress.clone();

                    let mut loaded = 0u32;
                    let mut c = 0;
                    for x in (chunk_coord.x-1)..(chunk_coord.x+1) {
                        for y in (chunk_coord.y-1)..(chunk_coord.y+1) {
                            for z in (chunk_coord.z-1)..(chunk_coord.z+1) {
                                if self.chunk_map.contains_key(&ChunkCoord::new(x, y, z)) {
                                    loaded |= 1 << c;
                                    c += 1;
                                }
                            }
                        }
                    }

                    let task = pool.spawn(async move {
                        generator.generate_chunk(loaded, chunk_coord, in_progress)
                    });
                    commands.spawn(ChunkBuildTask(task));
                }
            }
        }
    }

    pub fn build_chunk(&mut self, chunk_coord: ChunkCoord, chunk: Chunk) {
        self.chunk_map.insert(chunk_coord, chunk);
        self.needs_mesh_build.insert(chunk_coord);
        self.needs_chunk_build.remove(&chunk_coord);
    }

    pub fn queue_mesh_rebuild(
        &mut self,
        scanner: Query<&ChunkScanner>,
    ) {
        for (coord, chunk) in self.chunk_map.iter() {
            if chunk.needs_update() || (!chunk.is_empty() && scanner.single().should_load_mesh(coord) && !self.mesh_map.contains_key(coord)) {
                self.needs_mesh_build.insert(*coord);
            }
        }
    }

    pub fn build_meshes(
        &mut self,
        scanner: Query<&ChunkScanner>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut commands: Commands,
        texture_map: Res<TextureMapHandle>,
        texture_map_info: Res<TextureMapInfo>
    ) {
        let pool = AsyncComputeTaskPool::get();
        let task = pool.scope(|scope| {
            self.needs_mesh_build.drain_filter(|coord| {
                if let Some(chunk) = self.chunk_map.get(coord) {
                    if chunk.is_empty() {
                        return true;
                    }
                    if scanner.single().should_load_mesh(coord) {
                        if let Some(neighbors) = get_neighbors_data(&self.chunk_map, *coord) {
                            let info = &texture_map_info.0;
                            let data = chunk.get_data().as_ref().unwrap();
                            let coord = *coord;
                            scope.spawn(async move {
                                let (positions, normals, uvs, indices) = Chunk::gen_mesh(data, neighbors, info);
                                let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);
                                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                                mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
                                (coord, mesh)
                            });
    
                            return true;
                        }
                    }
                }
                false
            });
        });
    
        for (coord, mesh) in task {
            if let Some(mesh_handle) = self.mesh_map.get(&coord) {
                meshes.remove(mesh_handle);
            }
            let mesh_handle = meshes.add(mesh);
            self.mesh_map.insert(coord, mesh_handle.clone());

            commands.spawn(MaterialMeshBundle {
                mesh: mesh_handle,
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(texture_map.0.clone()),
                    reflectance: 0.0,
                    metallic: 0.0,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                transform: Transform::from_xyz(coord.x as f32 * CHUNK_SIZE.0 as f32, coord.y as f32 * CHUNK_SIZE.1 as f32, coord.z as f32 * CHUNK_SIZE.2 as f32),
                ..default()
            });

            self.chunk_map.get_mut(&coord).unwrap().set_updated();
        }
    }

    pub fn unload_chunks(
        &mut self,
        scanner: Query<&ChunkScanner>,
    ) {
        self.chunk_map.drain_filter(|coord, _chunk| {
            scanner.into_iter().fold(true, |unload, scanner| unload && scanner.should_unload_chunk(coord))
        });

        self.in_progress.retain(|coord, _| {
            !scanner.into_iter().fold(false, |retain, scanner| scanner.should_unload_unfinished_chunk(coord) || retain)
        });
    }

    pub fn unload_meshes(
        &mut self,
        scanner: Query<&ChunkScanner>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        self.mesh_map.drain_filter(|coord, _mesh| {
            !scanner.single().should_load_mesh(coord)
        }).into_iter().for_each(|(_, mesh)| {
            meshes.remove(mesh);
        });
    }

    pub fn get_block(&self, coord: &BlockCoord) -> Option<Block> {
        let (x,y,z) = (coord.x, coord.y, coord.z);
        let chunk_coord = ivec3(
            (x as f32 / CHUNK_SIZE.0 as f32).floor() as i32,
            (y as f32 / CHUNK_SIZE.1 as f32).floor() as i32,
            (z as f32 / CHUNK_SIZE.2 as f32).floor() as i32,
        );
        self.chunk_map.get(&chunk_coord).and_then(|chunk| chunk.get_block((
            (x - chunk_coord.x * CHUNK_SIZE.0 as i32) as usize,
            (y - chunk_coord.y * CHUNK_SIZE.1 as i32) as usize,
            (z - chunk_coord.z * CHUNK_SIZE.2 as i32) as usize,
        )))
    }

    pub fn set_block(&mut self, coord: &BlockCoord, block: Block) {
        let (x,y,z) = (coord.x, coord.y, coord.z);
        let chunk_coord = ivec3(
            (x as f32 / CHUNK_SIZE.0 as f32).floor() as i32,
            (y as f32 / CHUNK_SIZE.1 as f32).floor() as i32,
            (z as f32 / CHUNK_SIZE.2 as f32).floor() as i32,
        );
        match self.chunk_map.get_mut(&chunk_coord) {
            None => {
                error!("Tried to set block in unloaded chunk: {:?}", chunk_coord);
            },
            Some(chunk) => {
                if chunk.set_block((
                    (x - chunk_coord.x * CHUNK_SIZE.0 as i32) as usize,
                    (y - chunk_coord.y * CHUNK_SIZE.1 as i32) as usize,
                    (z - chunk_coord.z * CHUNK_SIZE.2 as i32) as usize,
                ), block) {
                    self.update_neighbors(chunk_coord);
                }
            },
        }
    }

    fn update_neighbors(&mut self, coord: ChunkCoord) {
        let neighbors = [ivec3(1,0,0) + coord, ivec3(-1,0,0) + coord, ivec3(0,-1,0) + coord, ivec3(0,1,0) + coord, ivec3(0,0,1) + coord, ivec3(0,0,-1) + coord];
        neighbors.into_iter().for_each(|coord| if let Some(chunk) = self.chunk_map.get_mut(&coord) {
            chunk.request_update();
        });
    }

    pub fn loaded_chunk_count(&self) -> usize {
        self.chunk_map.len()
    }
}

fn get_neighbors_data(chunk_map: &HashMap<ChunkCoord, Chunk>, coord: IVec3) -> Option<[Option<&Box<Array3<Block>>>;6]> {
    chunk_map.get(&(ivec3(1,0,0) + coord)).map(|chunk| chunk.get_data().as_ref()).and_then(|x| {
        chunk_map.get(&(ivec3(-1,0,0) + coord)).map(|chunk| chunk.get_data().as_ref()).and_then(|y| {
            chunk_map.get(&(ivec3(0,-1,0) + coord)).map(|chunk| chunk.get_data().as_ref()).and_then(|z| {
                chunk_map.get(&(ivec3(0,1,0) + coord)).map(|chunk| chunk.get_data().as_ref()).and_then(|w| {
                    chunk_map.get(&(ivec3(0,0,1) + coord)).map(|chunk| chunk.get_data().as_ref()).and_then(|u| {
                        chunk_map.get(&(ivec3(0,0,-1) + coord)).map(|chunk| chunk.get_data().as_ref()).map(|v| [x,y,z,w,u,v])
                    })
                })
            })
        })
    })
}

#[derive(Debug)]
pub struct UnfinishedChunkData {
    pub data: Option<Box<ndarray::Array3<Block>>>,
    pub block_list: Vec<((usize, usize, usize), Block)>,
    pub started: bool,
    pub finished: bool,
}
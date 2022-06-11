use bevy::utils::HashMap;
use crate::loader::*;

pub type ChunkMap = HashMap<ChunkCoord, Chunk>;

pub struct Worldgen {
    chunk_map: ChunkMap,
    mesh_map: HashMap<ChunkCoord, Handle<Mesh>>,
    texture_map_info: TextureMapInfo,
    generator: TerrainGenerator,
    needs_mesh_build: HashSet<ChunkCoord>,
    needs_chunk_build: HashSet<ChunkCoord>,
}

impl Worldgen {
    pub fn new(texture_map_info: TextureMapInfo, seed: u32) -> Self {
        Self {
            texture_map_info,
            generator: TerrainGenerator::new(seed),
            chunk_map: Default::default(),
            mesh_map: Default::default(),
            needs_mesh_build: Default::default(),
            needs_chunk_build: Default::default(),
        }
    }

    pub fn scan_chunks(&mut self, scanner: Query<&ChunkScanner>, pool: Res<AsyncComputeTaskPool>, mut commands: Commands,) {
        for scanner in scanner.iter() {
            for chunk_coord in scanner.into_iter() {
                if !self.chunk_map.contains_key(&chunk_coord) && !self.needs_chunk_build.contains(&chunk_coord) {
                    self.needs_chunk_build.insert(chunk_coord.clone());
                    let generator = self.generator.clone();
                    let task = pool.spawn(async move {
                        generator.gen(chunk_coord)
                    });
                    commands.spawn().insert(ChunkBuildTask(task));
                }
            }
        }
    }

    pub fn build_chunk(&mut self, coord: ChunkCoord, chunk: Chunk) {
        self.chunk_map.insert(coord.clone(), chunk);
        self.needs_mesh_build.insert(coord);
        self.needs_chunk_build.remove(&coord);
    }

    pub fn queue_mesh_rebuild(
        &mut self,
        scanner: Query<&ChunkScanner>,
    ) {
        for (coord, chunk) in self.chunk_map.iter() {
            if chunk.needs_update() || (scanner.single().should_load_mesh(coord) && !self.mesh_map.contains_key(coord)) {
                self.needs_mesh_build.insert(*coord);
            }
        }
    }

    pub fn build_meshes(
        &mut self,
        pool: Res<AsyncComputeTaskPool>,
        material_handle: Res<Handle<StandardMaterial>>,
        scanner: Query<&ChunkScanner>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut commands: Commands,
    ) {
        let task = pool.scope(|scope| {
            self.needs_mesh_build.drain_filter(|coord| {
                if let Some(chunk) = self.chunk_map.get(&coord) {
                    if !chunk.is_empty() && scanner.single().should_load_mesh(coord) {
                        if let Some(neighbors) = get_neighbors_data(&self.chunk_map, *coord) {
                            let info = &self.texture_map_info.info;
                            let data = chunk.get_data().as_ref().unwrap();
                            let coord = coord.clone();
                            scope.spawn(async move {
                                let (vertices, normals, texture_coords, indices) = Chunk::gen_mesh(&data, neighbors, info);
                                let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);
                                mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
                                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
                                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, texture_coords);
    
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
                meshes.remove(mesh_handle.id);
            }
            let mesh_handle = meshes.add(mesh);
            self.mesh_map.insert(coord, mesh_handle.clone());
            commands.spawn_bundle(MaterialMeshBundle {
                mesh: mesh_handle,
                material: material_handle.as_ref().clone(),
                transform: Transform::from_xyz(coord.x as f32 * CHUNK_SIZE.0 as f32, coord.y as f32 * CHUNK_SIZE.1 as f32, coord.z as f32 * CHUNK_SIZE.2 as f32),
                ..Default::default()
            });
            self.chunk_map.get_mut(&coord).unwrap().set_updated();
        }
    }

    pub fn unload_chunks(
        &mut self,
        scanner: Query<&ChunkScanner>,
    ) {
        self.chunk_map.drain_filter(|coord, _chunk| {
            scanner.single().should_unload_chunk(coord)
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
            meshes.remove(mesh.id);
        });
    }

    pub fn get_block(&self, coord: &BlockCoord) -> Option<Block> {
        let (x,y,z) = (coord.x, coord.y, coord.z);
        let chunk_coord = ivec3(
            (x as f32 / CHUNK_SIZE.0 as f32).floor() as i32,
            (y as f32 / CHUNK_SIZE.1 as f32).floor() as i32,
            (z as f32 / CHUNK_SIZE.2 as f32).floor() as i32,
        );
        match self.chunk_map.get(&chunk_coord) {
            None => {println!("CHUNK NOT FOUND {}", chunk_coord); None},
            Some(chunk) => chunk.get_block((
                (x - chunk_coord.x * CHUNK_SIZE.0 as i32) as usize,
                (y - chunk_coord.y * CHUNK_SIZE.1 as i32) as usize,
                (z - chunk_coord.z * CHUNK_SIZE.2 as i32) as usize,
            )),
        }
    }

    pub fn set_block(&mut self, coord: &BlockCoord, block: Block) {
        let (x,y,z) = (coord.x, coord.y, coord.z);
        let chunk_coord = ivec3(
            (x as f32 / CHUNK_SIZE.0 as f32).floor() as i32,
            (y as f32 / CHUNK_SIZE.1 as f32).floor() as i32,
            (z as f32 / CHUNK_SIZE.2 as f32).floor() as i32,
        );
        match self.chunk_map.get_mut(&chunk_coord) {
            None => (),
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

    fn update_neighbors(&mut self, coord: IVec3) {
        let neighbors = [ivec3(1,0,0) + coord, ivec3(-1,0,0) + coord, ivec3(0,-1,0) + coord, ivec3(0,1,0) + coord, ivec3(0,0,1) + coord, ivec3(0,0,-1) + coord];
        neighbors.into_iter().for_each(|coord| if let Some(chunk) = self.chunk_map.get_mut(&coord) {
            chunk.request_update();
        });
    }
}

fn get_neighbors_data(chunk_map: &HashMap<ChunkCoord, Chunk>, coord: IVec3) -> Option<[Option<&Box<Array3<Block>>>;6]> {
    Some([
        match chunk_map.get(&(ivec3(1,0,0) + coord)) {
            None => return None,
            Some(chunk) => chunk.get_data().as_ref(),
        },
        match chunk_map.get(&(ivec3(-1,0,0) + coord)) {
            None => return None,
            Some(chunk) => chunk.get_data().as_ref(),
        },
        match chunk_map.get(&(ivec3(0,-1,0) + coord)) {
            None => return None,
            Some(chunk) => chunk.get_data().as_ref(),
        },
        match chunk_map.get(&(ivec3(0,1,0) + coord)) {
            None => return None,
            Some(chunk) => chunk.get_data().as_ref(),
        },
        match chunk_map.get(&(ivec3(0,0,1) + coord)) {
            None => return None,
            Some(chunk) => chunk.get_data().as_ref(),
        },
        match chunk_map.get(&(ivec3(0,0,-1) + coord)) {
            None => return None,
            Some(chunk) => chunk.get_data().as_ref(),
        },
    ])
}
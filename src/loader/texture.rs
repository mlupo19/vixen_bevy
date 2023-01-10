use bevy::{utils::HashMap, prelude::{Handle, Image, Commands, Resource}};

use super::block_data::{RAW_BLOCK_DATA, TEXTURE_MAP_GRID_SIZE};

#[derive(Resource)]
pub struct TextureMapHandle(pub Handle<Image>);

#[derive(Resource)]
pub struct TextureMapInfo(pub HashMap<u16, [[[u32;2];4];6]>);

pub fn load_texture_map_info(
    mut commands: Commands,
) {
    let mut map = HashMap::new();
    for (id, unit) in RAW_BLOCK_DATA.blocks.iter() {
        let mut faces = [[[0;2];4];6];
        for (i, loc) in unit.loc.iter().enumerate() {
            let (y, x) = ((loc / RAW_BLOCK_DATA.grid) as u32, (loc % *TEXTURE_MAP_GRID_SIZE) as u32);
            faces[i] = [[x+1, y+1], [x+1, y], [x, y], [x, y+1]];
        }
        map.insert(*id, faces);
    }

    commands.insert_resource(TextureMapInfo(map));
}

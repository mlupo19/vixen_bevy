use bevy::{utils::HashMap, prelude::{Handle, Image, Commands, Res}};

use super::block_data::RawBlockData;

pub struct TextureMapHandle(pub Handle<Image>);

pub struct TextureMapInfo(pub HashMap<u16, [[[u32;2];4];6]>);

pub fn load_texture_map_info(
    mut commands: Commands,
    raw_block_data: Res<RawBlockData>,
) {
    let mut map = HashMap::new();
    for (id, unit) in raw_block_data.blocks.iter() {
        let mut faces = [[[0;2];4];6];
        for (i, loc) in unit.loc.iter().enumerate() {
            let (y, x) = ((loc / raw_block_data.grid) as u32, (loc % raw_block_data.grid) as u32);
            faces[i] = [[x+1, y+1], [x+1, y], [x, y], [x, y+1]];
        }
        map.insert(*id, faces);
    }

    commands.insert_resource(TextureMapInfo(map));
}

use bevy::utils::HashMap;
use std::fs::File;
use serde::{Deserialize, Serialize};

pub struct TextureMapInfo {
    pub info: HashMap<u16, [[[u32;2];4];6]>,
}

#[derive(Serialize, Deserialize)]
struct TextureMapUnit {
    name: String,
    loc: [u32;6],
}

#[derive(Serialize, Deserialize)]
struct TextureMapRawInfo {
    grid: u32,
    blocks: HashMap<u16, TextureMapUnit>,
}

pub fn load_texture_map_info(image_dimensions: (usize,usize)) -> TextureMapInfo {
    // Load texture map info

    let blocks = match File::open("assets/blocks.json") {
        Ok(blocks) => blocks,
        Err(e) => {
            panic!("Error opening blocks.json: {}", e);
        }
    };

    let info: TextureMapRawInfo = match serde_json::from_reader(blocks) {
        Ok(v) => v,
        Err(e) => {
            panic!("Error parsing blocks.json: {}",e);
        }
    };
    assert_eq!(image_dimensions.0, image_dimensions.1);
    
    // Process info
    let info = calculate(info.grid, info);

    TextureMapInfo { info }
}

#[inline]
fn calculate(grid_size: u32, info: TextureMapRawInfo) -> HashMap<u16, [[[u32;2];4];6]> {
    let mut map = HashMap::new();
    for (id, unit) in info.blocks {
        let mut faces = [[[0;2];4];6];
        for (i, loc) in unit.loc.iter().enumerate() {
            let (y, x) = ((loc / grid_size) as u32, (loc % grid_size) as u32);
            faces[i] = [[x+1, y+1], [x+1, y], [x, y], [x, y+1]];
        }
        map.insert(id, faces);
    }

    map
}
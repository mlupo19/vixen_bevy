use bevy::utils::HashMap;
use std::{fs::File, sync::Arc};
use serde::{Deserialize, Serialize};

pub struct TextureMapInfo {
    pub info: HashMap<u16, [[[f32;2];4];6]>,
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
    let unit_grid_size = image_dimensions.0 as u32 / info.grid;
    
    let info = calculate(unit_grid_size, info.grid, image_dimensions.0 as u32, info);

    TextureMapInfo { info }
}

#[inline]
fn calculate(unit_grid_size: u32, grid_size: u32, total_side_length: u32, info: TextureMapRawInfo) -> HashMap<u16, [[[f32;2];4];6]> {
    let mut map = HashMap::new();
    for (id, unit) in info.blocks {
        let mut faces = [[[0.0;2];4];6];
        for (i, loc) in unit.loc.iter().enumerate() {
            let (y, x) = ((loc / grid_size) as u32, (loc % grid_size) as u32);
            let (min_x, min_y, max_x, max_y) = (x as f32 * unit_grid_size as f32 / total_side_length as f32, y as f32 * unit_grid_size as f32 / total_side_length as f32, (x+1) as f32 * unit_grid_size as f32 / total_side_length as f32, (y+1) as f32 * unit_grid_size as f32 / total_side_length as f32);
            faces[i] = [[max_x, max_y], [max_x, min_y], [min_x, min_y], [min_x, max_y]];
        }
        map.insert(id, faces);
    }

    map
}
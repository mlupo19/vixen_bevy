use std::{fs::File, path::PathBuf};

use bevy::{utils::HashMap, prelude::{Handle, Image, Resource}, render::{render_resource::{Extent3d, TextureDimension}, texture::dds_format_to_texture_format}};
use ddsfile::{Dds, Error};
use serde::{Serialize, Deserialize};

use super::get_block_id;

#[derive(Resource)]
pub struct TextureMapHandle(pub Handle<Image>);

#[derive(Resource)]
pub struct TextureMapInfo(pub HashMap<u16, [[[f32;2];4];6]>);

pub fn gen_texture_map_info(face_map: HashMap<String, u32>, height: u32, num: u32) -> TextureMapInfo {
    let block_data = load_block_data("ghibli");

    let height = height as f32;
    let mut map = HashMap::new();
    for (name, block_textures) in block_data.iter() {
        let mut faces = [[[0.0;2];4];6];
        for i in 0..6 {
            let loc = face_map[block_textures.get(i)];
            let top = loc as f32 / num as f32;
            let bottom = (loc + 1) as f32 / num as f32;
            faces[i] = [[1.0, bottom - 0.5 / height], [1.0, top + 0.5 / height], [0.0, top + 0.5 / height], [0.0, bottom - 0.5 / height]];
            
        }
        let block_id = get_block_id(name).unwrap();
        map.insert(block_id, faces);
    }

    TextureMapInfo(map)
}


pub fn create_texture_map(path: &str) -> (Image, TextureMapInfo) {
    let files_in_dir = dds_files_in_dir(path);
    let mut images = Vec::new();
    let (mut width, mut height, mut num_layers, mut mipmap_levels, mut texture_format, mut depth) = (None, None, None, None, None, None);
    let mut face_map = HashMap::new();
    let mut id = 0;
    for file in files_in_dir {
        let Ok(dds) = load_dds(&file) else { continue; };
        if width.is_none() {
            width = Some(dds.get_width());
            height = Some(dds.get_height());
            num_layers = Some(dds.get_num_array_layers());
            texture_format = Some(dds_format_to_texture_format(&dds, true).unwrap());
            mipmap_levels = Some(dds.get_num_mipmap_levels());
            depth = Some(dds.get_depth());
        } else {
            assert_eq!(width, Some(dds.get_width()));
            assert_eq!(height, Some(dds.get_height()));
            assert_eq!(num_layers, Some(dds.get_num_array_layers()));
            assert_eq!(texture_format, Some(dds_format_to_texture_format(&dds, true).unwrap()));
            assert_eq!(mipmap_levels, Some(dds.get_num_mipmap_levels()));
            assert_eq!(depth, Some(dds.get_depth()));
        }
        images.push(dds);
        face_map.insert(file.file_stem().unwrap().to_str().unwrap().to_owned(), id);
        id += 1;
    }

    let mut image = Image::default();
    image.texture_descriptor.size = Extent3d {
        width: width.unwrap(),
        height: height.unwrap() * images.len() as u32,
        depth_or_array_layers: depth.unwrap(),
    }
    .physical_size(texture_format.unwrap());
    image.texture_descriptor.mip_level_count = mipmap_levels.unwrap();
    image.texture_descriptor.format = texture_format.unwrap();
    image.texture_descriptor.dimension = if depth.unwrap() > 1 {
        TextureDimension::D3
    } else if image.is_compressed() || height.unwrap() > 1 {
        TextureDimension::D2
    } else {
        TextureDimension::D1
    };

    // Stitch all the images together
    let mut data = Vec::new();
    let mipmap_levels = mipmap_levels.unwrap();
    for mipmap_level in 0..mipmap_levels {
        for image in images.iter() {
            let image_data = image.get_data(0).unwrap();
            let (start, end) = get_mipmap_size(image.get_main_texture_size().unwrap(), mipmap_level);
            data.extend_from_slice(&image_data[start..end]);
        }
    }
    image.data = data;
    
    (image, gen_texture_map_info(face_map, height.unwrap(), images.len() as u32))
}

fn dds_files_in_dir(path: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().unwrap() == "dds" {
            files.push(path);
        }
    }
    files
}

fn load_dds(path: &PathBuf) -> Result<Dds, Error> {
    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let dds = Dds::read(&mut reader);
    dds
}

fn get_mipmap_size(main_size: u32, mipmap_level: u32) -> (usize, usize) {
    let mut start = 0;
    let mut size = main_size;
    for _ in 0..mipmap_level {
        start += size;
        size /= 4;
    }
    (start as usize, (start + size) as usize)
}

#[derive(Serialize, Deserialize)]
pub struct BlockTextures {
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
    pub front: String,
    pub back: String,
}

pub fn load_block_data(data_pack: &str) -> HashMap<String, BlockTextures> {
    let path = format!("assets/packs/{}/blocks.json", data_pack);
    let blocks = match File::open(&path) {
        Ok(blocks) => blocks,
        Err(e) => {
            panic!("Error opening {}: {}", path, e);
        }
    };

    match serde_json::from_reader(blocks) {
        Ok(v) => v,
        Err(e) => {
            panic!("Error parsing blocks.json: {}",e);
        }
    }
}

impl BlockTextures {
    pub fn get(&self, id: usize) -> &str {
        match id {
            0 => &self.left,
            1 => &self.right,
            2 => &self.bottom,
            3 => &self.top,
            4 => &self.front,
            5 => &self.back,
            _ => panic!("Invalid face id"),
        }
    }
}
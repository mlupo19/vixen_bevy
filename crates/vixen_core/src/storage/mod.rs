use crate::loader::Block;
use bevy::prelude::Component;
use item::Item;

mod item;

#[derive(Default)]
pub enum StorageItem {
    Block(Block),
    Item(Item),
    #[default]
    Empty,
}

pub trait Storage {
    fn get_storage(&self) -> &StorageContainer;
    fn get_storage_mut(&mut self) -> &mut StorageContainer;
}

#[derive(Component)]
pub struct StorageContainer {
    data: Vec<StorageItem>,
    len: u32,
}

impl StorageContainer {
    pub fn new(len: u32) -> StorageContainer {
        Self { data: vec![], len }
    }

    pub fn get(&self, index: u32) -> &StorageItem {
        self.data.get(index as usize).unwrap_or(&StorageItem::Empty)
    }
}

use bevy::utils::HashMap;

pub struct Item {
    id: usize
}

fn init_item_map() -> HashMap<u16, ItemInfo> {
   HashMap::new()
}

pub struct ItemInfo;
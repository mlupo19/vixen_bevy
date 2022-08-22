use std::marker::PhantomData;

use super::block_data::get_multiplier;

pub trait Tool {
    // Get speed factor for tool based on block type
    fn get_speed_factor(&self, id: u16) -> f32;

    // Gets tool durability
    fn get_durability(&self) -> f32;
}

pub trait ToolRarity {
    // Get speed factor for rarity
    fn get_speed_factor(&self) -> f32;

    // Gets durability factor based on rarity
    fn get_durability_factor(&self) -> f32;
}

pub struct Wooden;

impl ToolRarity for Wooden {
    fn get_speed_factor(&self) -> f32 {
        1.0
    }

    fn get_durability_factor(&self) -> f32 {
        1.0
    }
}

pub struct Pickaxe<T> where T: ToolRarity {
    durability: f32,
    phantom_data: PhantomData<T>,
}

impl<T: ToolRarity> Default for Pickaxe<T> {
    fn default() -> Self {
        Self { durability: 1.0, phantom_data: PhantomData }
    }
}

impl Tool for Pickaxe<Wooden> {
    fn get_speed_factor(&self, id: u16) -> f32 {
        get_multiplier(id, "pickaxe")
    }

    fn get_durability(&self) -> f32 {
        self.durability
    }
}
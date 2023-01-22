pub trait BlockType: Send + Sync {
    fn get_name(&self) -> &'static str;
    fn get_id(&self) -> u16;
    fn get_durability(&self) -> f32;
}
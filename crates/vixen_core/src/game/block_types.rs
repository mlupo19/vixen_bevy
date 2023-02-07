pub trait BlockType: Send + Sync {
    fn get_name(&self) -> &'static str;
    fn get_durability(&self) -> f32;
    fn get_id(&self) -> u16;
    fn get_code_name(&self) -> &'static str;
}

use crate::graphics::resources::data::texture::TextureData;
use crate::graphics::types::id::GlobalTextureId;
use std::collections::HashMap;

pub struct TextureRegistry {
    next: GlobalTextureId,
    by_id: HashMap<GlobalTextureId, TextureData>,
}
impl TextureRegistry {
    pub fn new() -> Self {
        Self {
            next: 0,
            by_id: HashMap::new(),
        }
    }
    pub fn register(&mut self, tex: TextureData) -> GlobalTextureId {
        let id = self.next;
        self.next += 1;
        self.by_id.insert(id, tex);
        id
    }
    pub fn get(&self, id: GlobalTextureId) -> &TextureData {
        &self.by_id[&id]
    }
}

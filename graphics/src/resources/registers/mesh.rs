use crate::resources::data::mesh::MeshData;
use crate::types::id::GlobalMeshId;
use std::collections::HashMap;

pub struct MeshRegistry {
    next: GlobalMeshId,
    by_id: HashMap<GlobalMeshId, MeshData>,
}

impl MeshRegistry {
    pub fn new() -> Self {
        Self {
            next: 0,
            by_id: HashMap::new(),
        }
    }
    pub fn register(&mut self, mesh: MeshData) -> GlobalMeshId {
        let id = self.next;
        self.next += 1;
        self.by_id.insert(id, mesh);
        id
    }
    pub fn get(&self, id: GlobalMeshId) -> &MeshData {
        &self.by_id[&id]
    }
}

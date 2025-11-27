use crate::graphics::types::id::{GlobalMaterialId, GlobalMeshId};

#[derive(Clone)]
pub struct DrawItem {
    pub mesh: GlobalMeshId,
    pub material: GlobalMaterialId,
    pub transform: [[f32; 4]; 4],
}

impl DrawItem {
    pub fn new(mesh: GlobalMeshId, material: GlobalMaterialId, transform: [[f32; 4]; 4]) -> Self {
        Self {
            mesh,
            material,
            transform,
        }
    }
}

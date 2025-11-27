use crate::graphics::scene::camera::Camera;
use crate::graphics::types::id::GlobalCameraId;
use std::collections::HashMap;

pub struct CameraRegistry {
    next: GlobalCameraId,
    by_id: HashMap<GlobalCameraId, Camera>,
}

impl CameraRegistry {
    pub fn new() -> Self {
        Self {
            next: 0,
            by_id: HashMap::new(),
        }
    }
    pub fn register(&mut self, cam: Camera) -> GlobalCameraId {
        let id = self.next;
        self.next += 1;
        self.by_id.insert(id, cam);
        id
    }

    pub fn get(&self, id: GlobalCameraId) -> &Camera {
        self.by_id
            .get(&id)
            .expect(&format!("camera não encontrada {}", id))
    }
}

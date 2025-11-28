use crate::types::id::GlobalRenderTargetId;
use std::collections::HashMap;

#[derive(Clone)]
pub enum RenderTargetFormat {
    Rgba8,
    Rgba16f,
    Depth32,
}

#[derive(Clone)]
pub struct RenderTargetDesc {
    pub width: u32,
    pub height: u32,
    pub format: RenderTargetFormat,
    pub has_depth: bool, // criar depth buffer junto?
    pub sampled: bool,   // se será lido em inputs (criar sampler)
}

pub struct RenderTargetRegistry {
    next: GlobalRenderTargetId,
    by_id: HashMap<GlobalRenderTargetId, RenderTargetDesc>,
}

impl RenderTargetRegistry {
    pub fn new() -> Self {
        Self {
            next: 0,
            by_id: HashMap::new(),
        }
    }
    pub fn register(&mut self, desc: RenderTargetDesc) -> GlobalRenderTargetId {
        let id = self.next;
        self.next += 1;
        self.by_id.insert(id, desc);
        id
    }
    pub fn get(&self, id: GlobalRenderTargetId) -> &RenderTargetDesc {
        &self.by_id[&id]
    }
}

use crate::resources::desc::pipeline::PipelineDesc;
use crate::types::id::GlobalPipelineId;
use std::collections::HashMap;

pub struct PipelineRegistry {
    next: GlobalPipelineId,
    by_id: HashMap<GlobalPipelineId, PipelineDesc>,
}

impl PipelineRegistry {
    pub fn new() -> Self {
        Self {
            next: 0,
            by_id: HashMap::new(),
        }
    }
    pub fn register(&mut self, desc: PipelineDesc) -> GlobalPipelineId {
        let id = self.next;
        self.next += 1;
        self.by_id.insert(id, desc);
        id
    }
    pub fn get(&self, id: GlobalPipelineId) -> &PipelineDesc {
        self.by_id.get(&id).expect("pipeline id inválido")
    }
}

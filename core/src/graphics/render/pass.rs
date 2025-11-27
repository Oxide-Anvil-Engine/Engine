use crate::graphics::render::draw_item::DrawItem;
use crate::graphics::types::id::{GlobalCameraId, GlobalPipelineId, GlobalRenderTargetId};

pub enum LoadAction {
    Clear,
    Load,
}

pub struct RenderPassDesc {
    pub name: &'static str,
    pub pipeline: GlobalPipelineId,
    pub camera: GlobalCameraId,
    pub target: Option<GlobalRenderTargetId>, // None = backbuffer
    pub inputs: Vec<GlobalRenderTargetId>,    // textures lidas
    pub outputs: Vec<GlobalRenderTargetId>,   // MRT adicionais
    pub clear_color: [f32; 4],
    pub clear_depth: Option<f32>,
    pub load_color: LoadAction,
    pub load_depth: LoadAction,
    pub depends_on: Vec<&'static str>,
}

pub struct RenderPass {
    pub desc: RenderPassDesc,
    pub items: Vec<DrawItem>,
}

impl RenderPass {
    pub fn new(desc: RenderPassDesc) -> Self {
        Self {
            desc,
            items: Vec::new(),
        }
    }
    pub fn add(&mut self, item: DrawItem) {
        self.items.push(item);
    }
}

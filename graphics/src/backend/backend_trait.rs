use crate::backend::frame_ctx::FrameCtx;
use crate::render::pass::RenderPass;
use crate::resources::data::{mesh::MeshData, texture::TextureData};
use crate::resources::desc::pipeline::PipelineDesc;
use crate::scene::camera::Camera;
use crate::types::id::*;
use std::any::Any;

pub struct BackendOptions {
    pub power: Box<dyn Any>,
    pub features: Box<dyn Any>,
    pub limits: Box<dyn Any>,
    pub present_mode: Box<dyn Any>,
}

impl BackendOptions {
    pub fn new(
        power: Box<dyn Any>,
        features: Box<dyn Any>,
        limits: Box<dyn Any>,
        present_mode: Box<dyn Any>,
    ) -> Self {
        Self {
            power,
            features,
            limits,
            present_mode,
        }
    }
}

pub trait Backend<'w> {
    fn begin_frame(&mut self) -> FrameCtx;
    fn update_camera(&mut self, id: GlobalCameraId, matriz: &[[f32; 4]; 4]);
    fn draw_passes(&mut self, frame: &mut FrameCtx, passes: &[RenderPass]);
    fn end_frame(&mut self, frame: FrameCtx);

    // Ensures (lazy creation) of resources
    fn ensure_mesh(&mut self, id: GlobalMeshId, data: &MeshData);
    fn ensure_pipeline(&mut self, id: GlobalPipelineId, desc: &PipelineDesc);
    fn ensure_texture(&mut self, id: GlobalTextureId, data: &TextureData);
    fn ensure_camera(&mut self, id: GlobalCameraId, data: &Camera);

    fn resize(&mut self, w: u32, h: u32);
}

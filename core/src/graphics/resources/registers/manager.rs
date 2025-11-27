use crate::graphics::resources::registers::camera::CameraRegistry;
use crate::graphics::resources::registers::mesh::MeshRegistry;
use crate::graphics::resources::registers::pipeline::PipelineRegistry;
use crate::graphics::resources::registers::texture::TextureRegistry;

use crate::graphics::resources::data::mesh::MeshData;
use crate::graphics::resources::data::texture::TextureData;
use crate::graphics::resources::desc::pipeline::PipelineDesc;
use crate::graphics::scene::camera::Camera;

use crate::graphics::types::id::{GlobalCameraId, GlobalMeshId, GlobalPipelineId, GlobalTextureId};

pub struct ResourcesManager {
    pub(crate) mesh_registry: MeshRegistry,
    pub(crate) pipeline_registry: PipelineRegistry,
    pub(crate) texture_registry: TextureRegistry,
    pub(crate) camera_registry: CameraRegistry,

    pub(crate) used_meshes: Vec<GlobalMeshId>,
    pub(crate) used_pipelines: Vec<GlobalPipelineId>,
    pub(crate) used_textures: Vec<GlobalTextureId>,
    pub(crate) used_cameras: Vec<GlobalCameraId>,
}

impl ResourcesManager {
    pub fn new() -> Self {
        Self {
            mesh_registry: MeshRegistry::new(),
            pipeline_registry: PipelineRegistry::new(),
            texture_registry: TextureRegistry::new(),
            camera_registry: CameraRegistry::new(),

            used_meshes: Vec::new(),
            used_pipelines: Vec::new(),
            used_textures: Vec::new(),
            used_cameras: Vec::new(),
        }
    }

    pub fn get_mesh(&self, id: GlobalMeshId) -> &MeshData {
        self.mesh_registry.get(id)
    }

    pub fn get_pipeline(&self, id: GlobalPipelineId) -> &PipelineDesc {
        self.pipeline_registry.get(id)
    }

    pub fn get_texture(&self, id: GlobalTextureId) -> &TextureData {
        self.texture_registry.get(id)
    }

    pub fn get_camera(&self, id: GlobalCameraId) -> &Camera {
        self.camera_registry.get(id)
    }

    pub fn register_mesh(&mut self, mesh_data: MeshData) {
        self.mesh_registry.register(mesh_data);
    }

    pub fn register_pipeline(&mut self, pipeline_desc: PipelineDesc) {
        self.pipeline_registry.register(pipeline_desc);
    }

    pub fn register_texture(&mut self, texture_data: TextureData) {
        self.texture_registry.register(texture_data);
    }

    pub fn register_camera(&mut self, camera_data: Camera) {
        self.camera_registry.register(camera_data);
    }

    pub fn mark_mesh_used(&mut self, id: GlobalMeshId) {
        if !self.used_meshes.contains(&id) {
            self.used_meshes.push(id);
        }
    }

    pub fn mark_pipeline_used(&mut self, id: GlobalPipelineId) {
        if !self.used_pipelines.contains(&id) {
            self.used_pipelines.push(id);
        }
    }

    pub fn mark_texture_used(&mut self, id: GlobalTextureId) {
        if !self.used_textures.contains(&id) {
            self.used_textures.push(id);
        }
    }

    pub fn mark_camera_used(&mut self, id: GlobalCameraId) {
        if !self.used_cameras.contains(&id) {
            self.used_cameras.push(id);
        }
    }

    pub fn desmark_mesh_used(&mut self, id: GlobalMeshId) {
        self.used_meshes.retain(|&x| x != id);
    }

    pub fn desmark_pipeline_used(&mut self, id: GlobalPipelineId) {
        self.used_pipelines.retain(|&x| x != id);
    }

    pub fn desmark_texture_used(&mut self, id: GlobalTextureId) {
        self.used_textures.retain(|&x| x != id);
    }

    pub fn desmark_camera_used(&mut self, id: GlobalCameraId) {
        self.used_cameras.retain(|&x| x != id);
    }
}
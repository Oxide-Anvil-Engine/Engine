use crate::backend::backend_trait::Backend;
use crate::backend::frame_ctx::FrameCtx;
use crate::render::framegraph::FrameGraph;
use crate::render::pass::RenderPass;

use crate::resources::registers::manager::ResourcesManager;

use crate::backend::wgpu::backend_api::BackendWGPU;

pub struct Renderer {
    pub framegraph: FrameGraph,
    // registries externos (injetar ou referenciar):
    // mesh_registry, pipeline_registry, texture_registry, camera_registry
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            framegraph: FrameGraph::new(),
        }
    }

    pub fn add_pass(&mut self, pass: RenderPass) {
        self.framegraph.add_pass(pass);
    }

    pub fn build_graph(&mut self) {
        self.framegraph.build();
    }

    pub fn render(
        // deixar generico depois
        &mut self,
        backend: &mut BackendWGPU,
        resources_manager: &ResourcesManager,
    ) {
        let (used_meshes, used_pipelines, used_textures, used_cameras) =
            resources_manager.collect_used_resources();
        // 1. Ensure estáticos (só cria se faltar)
        for id in used_meshes {
            let data = resources_manager.get_mesh(*id);
            backend.ensure_mesh(*id, data);
        }
        for id in used_pipelines {
            let desc = resources_manager.get_pipeline(*id);
            backend.ensure_pipeline(*id, desc);
        }
        for id in used_textures {
            let tex = resources_manager.get_texture(*id);
            backend.ensure_texture(*id, tex);
        }

        // 2. begin_frame
        let mut frame = backend.begin_frame();
        if let FrameCtx::Skipped = frame {
            return;
        }

        // 3. Atualização de uniforms dinâmicos (câmeras)
        for id in used_cameras {
            let cam = resources_manager.get_camera(*id);
            let uni = cam.build_uniform();
            backend.update_camera(*id, &uni.view_proj);
        }

        // 4. Draw passes
        backend.draw_passes(&mut frame, &self.framegraph.passes_mut());

        // 5. end_frame
        backend.end_frame(frame);
    }
}

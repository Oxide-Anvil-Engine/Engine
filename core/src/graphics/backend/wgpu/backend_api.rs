use std::collections::HashMap;
use std::sync::Arc;

use crate::graphics::backend::backend_trait::Backend;
use crate::graphics::backend::frame_ctx::FrameCtx;
use crate::graphics::render::pass::RenderPass;

use crate::graphics::backend::backend_trait::BackendOptions;
use crate::graphics::backend::wgpu::camera::CameraWGPU;
use crate::graphics::backend::wgpu::mesh_cache::MeshWGPU;
use crate::graphics::types::id::{
    GlobalCameraId, GlobalMeshId, GlobalPipelineId, GlobalRenderTargetId,
};
use crate::wgpu::RenderPipeline as PipelineWGPU;

// Importa wgpu
use wgpu::{
    Adapter, CommandEncoder, Device, Instance, Queue, Surface, SurfaceConfiguration,
    SurfaceTexture, TextureView,
};

use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub struct BackendOptionsWGPU {
    pub power: wgpu::PowerPreference,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
    pub present_mode: wgpu::PresentMode,
}

impl From<BackendOptionsWGPU> for BackendOptions {
    fn from(opts: BackendOptionsWGPU) -> Self {
        Self {
            power: Box::new(opts.power),
            features: Box::new(opts.features),
            limits: Box::new(opts.limits),
            present_mode: Box::new(opts.present_mode),
        }
    }
}

impl Default for BackendOptionsWGPU {
    fn default() -> Self {
        Self {
            power: wgpu::PowerPreference::HighPerformance,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            present_mode: wgpu::PresentMode::Fifo,
        }
    }
}

pub struct BackendWGPU {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub config: SurfaceConfiguration,
    pub instance: Instance,
    pub adapter: Adapter,
    pub mesh_cache: HashMap<GlobalMeshId, MeshWGPU>,
    pub pipeline_cache: HashMap<GlobalPipelineId, PipelineWGPU>,
    pub camera_cache: HashMap<GlobalCameraId, CameraWGPU>,
    pub rt_cache: HashMap<GlobalRenderTargetId, RenderTargetWGPU>,
}

pub struct FrameCtxWGPU {
    pub output: SurfaceTexture,
    pub view: TextureView,
    pub depth_view: Option<TextureView>,
    pub encoder: CommandEncoder,
}

impl From<FrameCtxWGPU> for FrameCtx {
    fn from(ctx: FrameCtxWGPU) -> Self {
        FrameCtx::new(ctx)
    }
}

pub struct RenderTargetWGPU {
    pub color_tex: wgpu::Texture, // textura bruta onde pixels são escritos
    pub color_view: wgpu::TextureView, // view usada como attachment ou sampled
    pub depth_view: Option<wgpu::TextureView>, // view do depth (se has_depth)
    pub sampler: Option<wgpu::Sampler>, // sampler para leitura em shaders (se sampled)
}

impl Backend<'static> for BackendWGPU {
    fn begin_frame(&mut self) -> FrameCtx {
        let output = match self.surface.get_current_texture() {
            Ok(o) => o,
            Err(_) => {
                // Frame skip simples
                return FrameCtx::skip();
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame"),
            });
        let depth_view = None;
        FrameCtx::from(FrameCtxWGPU {
            output,
            view,
            depth_view,
            encoder,
        })
    }

    fn update_camera(&mut self, id: GlobalCameraId, matriz: &[[f32; 4]; 4]) {
        if let Some(camera) = self.camera_cache.get_mut(&id) {
            camera.update_uniform(&self.queue, matriz);
        }
    }

    fn draw_passes(&mut self, frame: &mut FrameCtx, passes: &[RenderPass]) {
        let f = frame.as_mut::<FrameCtxWGPU>();
        if f.is_none() {
            return;
        }
        let f = f.unwrap();

        for p in passes {
            let desc = &p.desc;

            // Resolve o target: None = backbuffer (view do frame)
            // Some(rt_id) = render target offscreen do cache do backend
            let (color_view, depth_view_opt) = if let Some(rt_id) = desc.target {
                // rt_cache deve mapear GlobalRenderTargetId -> RenderTargetWGPU
                let rt = self
                    .rt_cache
                    .get(&rt_id)
                    .expect("render target não criado (ensure_render_target)");
                (&rt.color_view, rt.depth_view.as_ref())
            } else {
                (&f.view, None)
            };

            // Load/Clear para color
            let load_color = match desc.load_color {
                crate::graphics::render::pass::LoadAction::Clear => {
                    let c = desc.clear_color;
                    wgpu::LoadOp::Clear(wgpu::Color {
                        r: c[0] as f64,
                        g: c[1] as f64,
                        b: c[2] as f64,
                        a: c[3] as f64,
                    })
                }
                crate::graphics::render::pass::LoadAction::Load => wgpu::LoadOp::Load,
            };

            // Load/Clear para depth (se existir depth nesse pass/target)
            let depth_ops = desc.clear_depth.map(|d| {
                let load = match desc.load_depth {
                    crate::graphics::render::pass::LoadAction::Clear => wgpu::LoadOp::Clear(d),
                    crate::graphics::render::pass::LoadAction::Load => wgpu::LoadOp::Load,
                };
                wgpu::Operations {
                    load,
                    store: wgpu::StoreOp::Store,
                }
            });

            // Abre o render pass
            let mut rp = f.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(desc.name),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: load_color,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: depth_view_opt.map(|dv| {
                    wgpu::RenderPassDepthStencilAttachment {
                        view: dv,
                        depth_ops,
                        stencil_ops: None,
                    }
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Pipeline
            if let Some(pipe) = self.pipeline_cache.get(&desc.pipeline) {
                rp.set_pipeline(pipe);
            } else {
                // Sem pipeline válido para esse pass — nada a desenhar
                continue;
            }

            // Bind da câmera (slot 0)
            if let Some(cam) = self.camera_cache.get(&desc.camera) {
                rp.set_bind_group(0, &cam.bind_group, &[]);
            }

            // TODO opcional: bindings para inputs (textures lidas) em group(1)
            // self.bind_inputs(&mut rp, &desc.inputs);

            // Desenho dos itens
            for it in &p.items {
                if let Some(mesh) = self.mesh_cache.get(&it.mesh) {
                    rp.set_vertex_buffer(0, mesh.vertex_buf.slice(..));
                    rp.set_index_buffer(mesh.index_buf.slice(..), mesh.index_format);
                    rp.draw_indexed(0..mesh.index_count, 0, 0..1);
                }
            }
            // rp é dropado ao sair do escopo → encerra o render pass
        }
    }

    fn end_frame(&mut self, frame: FrameCtx) {
        if let FrameCtx::Skipped = frame {
            return;
        }
        let f = frame.into_inner::<FrameCtxWGPU>().expect("wgpu frame");
        self.queue.submit(std::iter::once(f.encoder.finish()));
        f.output.present();
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    fn ensure_camera(
        &mut self,
        id: crate::graphics::types::id::GlobalCameraId,
        data: &crate::graphics::scene::camera::Camera,
    ) {
        let camera = self.camera_cache.get(&id);
        if camera.is_some() {
            return;
        }
        let mut cam_wgpu = CameraWGPU::new(&self.device);
        cam_wgpu.update_uniform(&self.queue, &data.build_uniform().view_proj);
        self.camera_cache.insert(id, cam_wgpu);
    }

    fn ensure_mesh(
        &mut self,
        _id: crate::graphics::types::id::GlobalMeshId,
        _data: &crate::graphics::resources::data::mesh::MeshData,
    ) {
    }

    fn ensure_pipeline(
        &mut self,
        _id: crate::graphics::types::id::GlobalPipelineId,
        _data: &crate::graphics::resources::desc::pipeline::PipelineDesc,
    ) {
    }

    fn ensure_texture(
        &mut self,
        _id: crate::graphics::types::id::GlobalTextureId,
        _data: &crate::graphics::resources::data::texture::TextureData,
    ) {
    }
}

impl BackendWGPU {
    pub fn new(
        window: Arc<winit::window::Window>,
        size: (u32, u32),
        opts: BackendOptionsWGPU,
    ) -> Self {
        let instance = wgpu::Instance::default();
        // Modo adaptado do wgpu de criar surface
        let wh = window.window_handle().expect("wh").as_raw();
        let dh = window.display_handle().expect("dh").as_raw();
        let surface = unsafe {
            instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_window_handle: wh,
                raw_display_handle: dh,
            })
        }
        .expect("surface");
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: opts.power,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("adapter");
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("device"),
            required_features: opts.features & adapter.features(),
            required_limits: opts.limits.clone().using_resolution(adapter.limits()),
            memory_hints: wgpu::MemoryHints::default(),
            experimental_features: wgpu::ExperimentalFeatures::default(),
            trace: wgpu::Trace::default(),
        }))
        .expect("device");
        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.0,
            height: size.1,
            present_mode: opts.present_mode,
            alpha_mode: caps.alpha_modes[0],
            view_formats: Vec::new(),
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        Self {
            device,
            queue,
            surface,
            config,
            instance,
            adapter,
            mesh_cache: HashMap::new(),
            pipeline_cache: HashMap::new(),
            camera_cache: HashMap::new(),
            rt_cache: HashMap::new(),
        }
    }
}

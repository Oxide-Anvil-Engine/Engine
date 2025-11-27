use crate::graphics::resources::desc::pipeline::PipelineDesc;
use crate::graphics::types::id::GlobalPipelineId;
use wgpu::*;

pub struct PipelineWGPU {
    pub pipeline: RenderPipeline,
}

pub struct PipelineCacheWGPU {
    map: std::collections::HashMap<GlobalPipelineId, PipelineWGPU>,
    pub camera_layout: BindGroupLayout,
    pub color_format: wgpu::TextureFormat,
}

impl PipelineCacheWGPU {
    pub fn new(device: &Device, backbuffer_format: wgpu::TextureFormat) -> Self {
        let camera_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("camera_layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let color_format = backbuffer_format;
        Self {
            map: Default::default(),
            camera_layout,
            color_format,
        }
    }
    pub fn get(&self, id: GlobalPipelineId) -> Option<&PipelineWGPU> {
        self.map.get(&id)
    }

    pub fn ensure(&mut self, id: GlobalPipelineId, desc: &PipelineDesc, device: &Device) {
        if self.map.contains_key(&id) {
            return;
        }
        let vs_mod = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("vs"),
            source: ShaderSource::Wgsl(desc.vertex_shader.clone().into()),
        });
        let fs_mod = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("fs"),
            source: ShaderSource::Wgsl(desc.fragment_shader.clone().into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts: &[&self.camera_layout],
            push_constant_ranges: &[],
        });

        let array_stride = if desc.is_tridimensional {
            std::mem::size_of::<crate::graphics::resources::data::mesh::Vertex3D>() as u64
        } else {
            std::mem::size_of::<crate::graphics::resources::data::mesh::Vertex2D>() as u64
        };
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &vs_mod,
                entry_point: Some(&desc.vs_entry),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        // pos
                        VertexAttribute {
                            format: VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 0,
                        },
                        // color
                        VertexAttribute {
                            format: VertexFormat::Uint32,
                            offset: 12,
                            shader_location: 1,
                        },
                    ],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &fs_mod,
                entry_point: Some(&desc.fs_entry),
                targets: &[Some(ColorTargetState {
                    format: self.color_format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: if desc.depth {
                Some(DepthStencilState {
                    format: TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: CompareFunction::Less,
                    stencil: Default::default(),
                    bias: Default::default(),
                })
            } else {
                None
            },
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        self.map.insert(id, PipelineWGPU { pipeline });
    }
}

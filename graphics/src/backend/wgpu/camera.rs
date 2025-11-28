use crate::resources::uniform_blocks::CameraUniform;

pub struct CameraWGPU {
    // 1. O Buffer de Dados (Na GPU)
    pub(crate) buffer: wgpu::Buffer,

    // 2. O Bind Group (@group(0))
    pub(crate) bind_group: wgpu::BindGroup,

    // 3. A Estrutura de Layout (Define o contrato)
    pub(crate) _layout: wgpu::BindGroupLayout,

    // 4. Os Dados na CPU (Para atualização)
    pub(crate) uniform: CameraUniform,
}

impl CameraWGPU {
    pub fn new(device: &wgpu::Device) -> Self {
        // --- A. Layout (O Contrato) ---
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
            entries: &[
                // Entrada 0: Uniform Buffer da Câmera (Matrizes)
                wgpu::BindGroupLayoutEntry {
                    binding: 0, // slot @binding(0)

                    // 💡 visibility: Diz em quais estágios do shader este recurso é usado.
                    // A matriz da Câmera é usada quase sempre no Vertex Shader para transformar a posição.
                    visibility: wgpu::ShaderStages::VERTEX,

                    // 💡 ty: O tipo de recurso (Uniform Buffer)
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform, // É um Buffer Uniforme
                        has_dynamic_offset: false, // Não estamos usando offsets dinâmicos
                        min_binding_size: None,    // Sem requisito mínimo de tamanho
                    },
                    count: None, // Não é um array de Bindings
                },
            ],
        });

        // --- B. Buffer (Onde os dados vivem) ---
        let uniform = CameraUniform::new();
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera_buffer"),
            size: std::mem::size_of::<CameraUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false, // false para dinamico (atualizado via queue.write_buffer)
        });

        // --- C. Bind Group (O Ponto de Acesso) ---
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &layout, // Usa o layout que acabamos de criar
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(), // Liga o buffer da câmera
            }],
        });

        Self {
            buffer,
            bind_group,
            _layout: layout,
            uniform,
        }
    }

    pub fn update_uniform(&mut self, queue: &wgpu::Queue, view_proj: &[[f32; 4]; 4]) {
        self.uniform.update_from_matriz(view_proj);
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}

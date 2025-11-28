use crate::resources::data::mesh::VertexFormat;
use crate::resources::data::mesh::VertexLayoutDesc;

pub fn to_wgpu_layout(desc: &VertexLayoutDesc) -> wgpu::VertexBufferLayout<'static> {
    use wgpu::VertexFormat as WF;
    fn map(f: VertexFormat) -> WF {
        match f {
            VertexFormat::F32x2 => WF::Float32x2,
            VertexFormat::F32x3 => WF::Float32x3,
            VertexFormat::Unorm8x4 => WF::Unorm8x4,
        }
    }
    let attrs: Vec<wgpu::VertexAttribute> = desc
        .attributes
        .iter()
        .map(|a| wgpu::VertexAttribute {
            format: map(a.format),
            offset: a.offset as wgpu::BufferAddress,
            shader_location: a.location,
        })
        .collect();
    wgpu::VertexBufferLayout {
        array_stride: desc.stride as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: Box::leak(attrs.into_boxed_slice()),
    }
}

use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Debug)]
pub enum VertexFormat {
    F32x2,
    F32x3,
    Unorm8x4,
}

impl VertexFormat {
    pub fn size(&self) -> u32 {
        match self {
            VertexFormat::F32x2 => 8,
            VertexFormat::F32x3 => 12,
            VertexFormat::Unorm8x4 => 4,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VertexAttributeDesc {
    pub location: u32,
    pub offset: u32,
    pub format: VertexFormat,
}

pub trait VertexSpec {
    fn stride() -> u32;
    fn attributes() -> &'static [VertexAttributeDesc];
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex2D {
    pub pos: [f32; 2],
    pub color: [u8; 4],
}

impl VertexSpec for Vertex2D {
    fn stride() -> u32 {
        std::mem::size_of::<Vertex2D>() as u32
    }
    fn attributes() -> &'static [VertexAttributeDesc] {
        static ATTRS: [VertexAttributeDesc; 2] = [
            VertexAttributeDesc {
                location: 0,
                offset: 0,
                format: VertexFormat::F32x2,
            },
            VertexAttributeDesc {
                location: 1,
                offset: 8,
                format: VertexFormat::Unorm8x4,
            },
        ];
        &ATTRS
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex3D {
    pub pos: [f32; 3],
    pub color: [u8; 4],
}

impl VertexSpec for Vertex3D {
    fn stride() -> u32 {
        std::mem::size_of::<Vertex3D>() as u32
    }
    fn attributes() -> &'static [VertexAttributeDesc] {
        static ATTRS: [VertexAttributeDesc; 2] = [
            VertexAttributeDesc {
                location: 0,
                offset: 0,
                format: VertexFormat::F32x3,
            },
            VertexAttributeDesc {
                location: 1,
                offset: 12,
                format: VertexFormat::Unorm8x4,
            },
        ];
        &ATTRS
    }
}

#[derive(Clone, Debug)]
pub struct VertexLayoutDesc {
    pub stride: u32,
    pub attributes: Vec<VertexAttributeDesc>,
}

#[derive(Clone, Debug)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl Indices {
    pub fn len(&self) -> usize {
        match self {
            Indices::U16(v) => v.len(),
            Indices::U32(v) => v.len(),
        }
    }

    pub fn as_bytes_format_count(&self) -> (&[u8], wgpu::IndexFormat, u32) {
        match self {
            Indices::U16(v) => (
                bytemuck::cast_slice(v),
                wgpu::IndexFormat::Uint16,
                v.len() as u32,
            ),
            Indices::U32(v) => (
                bytemuck::cast_slice(v),
                wgpu::IndexFormat::Uint32,
                v.len() as u32,
            ),
        }
    }
}

#[derive(Clone)]
pub struct MeshData {
    pub vertex_bytes: Vec<u8>,
    pub layout: VertexLayoutDesc,
    pub indices: Indices,
}

impl MeshData {
    pub fn new<V: Pod + VertexSpec>(vertices: &[V], indices: Indices) -> Self {
        let vb = Vec::from(bytemuck::cast_slice(vertices));
        Self {
            vertex_bytes: vb,
            layout: VertexLayoutDesc {
                stride: V::stride(),
                attributes: V::attributes().to_vec(),
            },
            indices,
        }
    }

    pub fn make_square(size: f32, color: [u8; 4]) -> Self {
        let h = size * 0.5;
        let verts = [
            Vertex2D {
                pos: [-h, -h],
                color,
            },
            Vertex2D {
                pos: [h, -h],
                color,
            },
            Vertex2D { pos: [h, h], color },
            Vertex2D {
                pos: [-h, h],
                color,
            },
        ];
        let idx: Vec<u16> = vec![0, 1, 2, 0, 2, 3];
        Self::new(&verts, Indices::U16(idx))
    }

    pub fn make_cube(size: f32, color: [u8; 4]) -> Self {
        let h = size * 0.5;
        let verts = [
            Vertex3D {
                pos: [-h, -h, h],
                color,
            },
            Vertex3D {
                pos: [h, -h, h],
                color,
            },
            Vertex3D {
                pos: [h, h, h],
                color,
            },
            Vertex3D {
                pos: [-h, h, h],
                color,
            },
            Vertex3D {
                pos: [-h, -h, -h],
                color,
            },
            Vertex3D {
                pos: [h, -h, -h],
                color,
            },
            Vertex3D {
                pos: [h, h, -h],
                color,
            },
            Vertex3D {
                pos: [-h, h, -h],
                color,
            },
        ];
        let idx: Vec<u16> = vec![
            0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 4, 5, 1, 4, 1, 0, 3, 2, 6, 3, 6, 7, 1, 5, 6, 1, 6,
            2, 4, 0, 3, 4, 3, 7,
        ];
        Self::new(&verts, Indices::U16(idx))
    }
}

impl From<Vec<u16>> for Indices {
    fn from(v: Vec<u16>) -> Self {
        Indices::U16(v)
    }
}
impl From<Vec<u32>> for Indices {
    fn from(v: Vec<u32>) -> Self {
        Indices::U32(v)
    }
}

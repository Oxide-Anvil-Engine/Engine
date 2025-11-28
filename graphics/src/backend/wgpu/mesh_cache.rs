use crate::resources::data::mesh::MeshData;
use crate::types::id::GlobalMeshId;
use wgpu::util::DeviceExt;
use wgpu::*;

#[derive(Debug)]
pub struct MeshWGPU {
    pub vertex_buf: Buffer,
    pub index_buf: Buffer,
    pub index_count: u32,
    pub index_format: IndexFormat,
}

pub struct MeshCacheWGPU {
    map: std::collections::HashMap<GlobalMeshId, MeshWGPU>,
}

impl MeshCacheWGPU {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
    pub fn get(&self, id: GlobalMeshId) -> Option<&MeshWGPU> {
        self.map.get(&id)
    }

    pub fn ensure(&mut self, id: GlobalMeshId, data: &MeshData, device: &Device, _queue: &Queue) {
        if self.map.contains_key(&id) {
            return;
        }

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh_vb"),
            contents: &data.vertex_bytes, // se você usar Vertex tipado, troque por cast_slice(&data.vertices)
            usage: BufferUsages::VERTEX,
        });

        // suporte u16/u32
        let (index_bytes, index_format, index_count) = data.indices.as_bytes_format_count();

        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh_ib"),
            contents: index_bytes,
            usage: BufferUsages::INDEX,
        });

        let entry = MeshWGPU {
            vertex_buf,
            index_buf,
            index_count,
            index_format,
        };
        self.map.insert(id, entry);
    }
}

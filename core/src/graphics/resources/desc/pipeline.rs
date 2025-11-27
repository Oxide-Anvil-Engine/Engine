

#[derive(Clone)]
pub struct PipelineDesc {
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub vs_entry: String,
    pub fs_entry: String,
    pub depth: bool,
    pub is_tridimensional: bool,
}
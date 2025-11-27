#[derive(Clone)]
pub struct ShaderDesc {
    pub vertex_path: String,
    pub fragment_path: String,
    pub vs_entry: String,
    pub fs_entry: String,
    pub defines: Vec<(String, String)>,
}

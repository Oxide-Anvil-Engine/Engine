// Engine/core/src/shaders/shader.wgsl
// =======================================================================================
// ESTRUTURAS DE DADOS (Conectando Rust <-> Shader)
// =======================================================================================

// Estrutura que define o formato do Uniform Buffer acima
struct CameraUniform {
    view_proj: mat4x4<f32>, 
};

// 1. DADOS DE ENTRADA UNIFORME (Do Rust, Grupo 0, Binding 0)
// Este bloco contém a matriz View * Projection para a Câmera.
@group(0) @binding(0)
var<uniform> camera: CameraUniform;


// 2. [ENTRADA do Vertex Shader]
// Corresponde à sua struct 'Vertex' em Rust.
struct VertexInput {
    // @location(0): Posição 3D (x, y, z)
    @location(0) position: vec3<f32>, 
    
    // @location(1): Cor (r, g, b)
    @location(1) color: vec3<f32>, 
};


// 3. [SAÍDA do Vertex Shader / ENTRADA do Fragment Shader]
// Define os dados a serem interpolados pelo hardware.
struct FragmentInput {
    // @builtin(position): Posição final na tela (clip position). OBRIGATÓRIO.
    @builtin(position) clip_position: vec4<f32>, 
    
    // @location(0): Cor interpolada para o Fragment Shader.
    @location(0) fragment_color: vec3<f32>,
};


// =======================================================================================
// VERTEX SHADER (@vertex) - Onde a Magia 3D Acontece
// =======================================================================================

// Função principal que é executada para CADA VÉRTICE.
@vertex
fn vs_main(
    model: VertexInput,
) -> FragmentInput {
    var out: FragmentInput;
    
    // ➡️ CORREÇÃO CRUCIAL: APLICAÇÃO DA MATRIZ DA CÂMERA ⬅️
    // Multiplicamos a matriz View-Projection pela posição 3D do vértice.
    // O 'vec4<f32>(..., 1.0)' transforma a posição 3D em 4D homogênea.
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);

    // Passagem de Cor: (Inalterado, pois a cor não precisa ser transformada)
    out.fragment_color = model.color;
    
    return out;
}


// =======================================================================================
// FRAGMENT SHADER (@fragment) - Onde a Cor é Definida
// =======================================================================================

// Função principal que é executada para CADA PIXEL (fragmento).
@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    // Retorna a cor interpolada que veio do Vertex Shader, adicionando o canal Alpha (1.0).
    return vec4<f32>(in.fragment_color, 1.0); 
}
use bytemuck::{Pod, Zeroable};

// --- DEFINIÇÃO DA ESTRUTURA ---

// O atributo 'repr(C)' garante que os campos da estrutura (neste caso, view_proj)
// sejam dispostos na memória exatamente como em uma linguagem C.
// Isso é crucial para que a GPU, que usa padrões C, leia os dados corretamente
// do Uniform Buffer.
#[repr(C)]
// 'Copy' permite que a estrutura seja copiada rapidamente (por valor),
// em vez de ser movida, o que é eficiente para dados pequenos de Uniform Buffer.
#[derive(Copy)]
// 'Clone' permite criar uma cópia da estrutura, exigido por Copy.
#[derive(Clone)]
// 'Debug' permite que a estrutura seja formatada para depuração (saída de console).
#[derive(Debug)]
// O trait 'Pod' (Plain Old Data) afirma que a estrutura é segura para ser lida
// bit a bit pela GPU, sem referências internas ou padding complexo.
#[derive(Pod)]
// O trait 'Zeroable' garante que a estrutura pode ser inicializada com zeros
// (todos os seus bits são zero), o que é necessário para o wgpu::util::DeviceExt
// e para o bytemuck.
#[derive(Zeroable)]
pub struct CameraUniform {
    // view_proj é a Matriz de Projeção e Visualização combinada.
    // Ela transforma as coordenadas do mundo para as Coordenadas de Clipping da GPU.
    // O formato [[f32; 4]; 4] é o array de arrays padrão para um mat4x4.
    pub view_proj: [[f32; 4]; 4],
}

// --- IMPLEMENTAÇÃO DA ESTRUTURA ---

impl CameraUniform {
    // Função new (construtor) para inicializar a estrutura.
    pub fn new() -> Self {
        Self {
            // Inicializa a matriz com todos os elementos zerados.
            // É uma boa prática inicializar Uniform Buffers, mesmo que zeros
            // não representem uma matriz de identidade funcional.
            view_proj: [[0.0; 4]; 4],
        }
    }

    pub fn update_from_matriz(&mut self, matriz: &[[f32; 4]; 4]) {
        self.view_proj = *matriz;
    }
}

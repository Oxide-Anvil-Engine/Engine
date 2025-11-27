# OBJETIVOS — plano estendido (2D → 2.5D → 3D) e Nodes instanciáveis

Este documento lista metas e passos práticos para evoluir a engine do núcleo 2D até capacidades 2.5D e 3D, incluindo movimento, colisão e um sistema de Nodes que permita instanciamento e reutilização (prefabs/instancing).

Formato deste documento
- Para cada fase eu descrevo: objetivo, entregáveis, passos detalhados (sub-tarefas) e critérios de aceitação.

FASE A — Base 2D (fundação)
Objetivo
- Ter uma pipeline 2D mínima, Nodes funcionais e sistemas básicos de movimento/colisão.

Entregáveis
- `State` wgpu que inicializa GPU e limpa a tela.
- `Node` básico com `Transform2D` e um componente `Sprite` que renderiza um quad.
- Sistema de atualização (tick): aplica `Velocity` e resolve colisões AABB simples.

Passos detalhados
1. Criar `graphics::State` (wgpu) e exemplo `wgpu_clear`.
   - inicializar `Instance`, `Adapter`, `Device`, `Queue`, `Surface` e `SurfaceConfiguration`.
   - apresentar frame (clear color).
2. Implementar `Node` e `NodeManager` básicos.
   - `Node { id, parent, children, transform: Transform2D, components }`.
   - API: `create_node`, `destroy_node`, `add_component`, `remove_component`, `iterate_nodes`.
3. Criar componentes essenciais.
   - `Transform2D { position: Vec2, rotation: f32, scale: Vec2 }`.
   - `Sprite { texture: Option<TextureId>, color: Color }`.
   - `Velocity { vel: Vec2 }`.
   - `ColliderAABB { half_extents: Vec2 }`.
4. Implementar sistema de update por tick.
   - tickrate configurável (ex.: 60 Hz), `update(dt)` aplica velocity e atualiza transforms.
5. Implementar colisão AABB e resolução simples.
   - detecção AABB vs AABB; correção por separação mínima; aplicar knockback se necessário.

Critérios de aceitação
- Exemplo demonstra dois nodes com sprites movendo-se e colidindo (visual + logs).

Tempo estimado: 1–2 semanas

FASE B — Render 2D avançado e ferramentas de produtividade
Objetivo
- Melhorar render 2D: batching, atlas de sprites, câmera, e hot-reload de shaders.

Entregáveis
- Sprite batching (reduzir draw calls).
- Atlas de texturas e gerador de atlas simples.
- Câmera ortográfica com view/proj UBO.
- Hot-reload WGSL para pipelines em desenvolvimento.

Passos detalhados
1. Implementar vertex/layout para sprites (pos, uv, color, texture_id).
2. Implementar atlas de texturas e sistema de binding por slot.
3. Implementar batching por material/texture_id.
4. Adicionar câmera 2D com UBO e bind group (CameraGroup já existente pode ser usada).
5. Implementar watcher de arquivos para recarregar WGSL e recriar pipelines sem reiniciar.

Critérios de aceitação
- Renderiza dezenas de sprites mantendo taxa de frames estável; mudança de shader em tempo real atualiza rendering.

Tempo estimado: 1–2 semanas

FASE C — 2.5D (profundidade visual, layer/instancing)
Objetivo
- Suportar efeitos que simulam profundidade sem introduzir full 3D: parallax, layers, simples instancing, sort by depth.

Entregáveis
- Sistema de layers e parallax.
- Instancing para muitos objetos similares (mesmo mesh/quad com diferentes transforms).
- Depth (Z) simples para ordenação e opcional depth test em offscreen target.

Passos detalhados
1. Adicionar `z` ao `Transform2D` (ou Transform2.5) e usar para sort e parallax.
2. Implementar instanced rendering para sprites que compartilham material.
3. Adicionar offscreen depth texture opcional e testes para pequenos passes que requerem depth.

Critérios de aceitação
- Exemplo com parallax background, middle ground, foreground, e centenas de instanced sprites.

Tempo estimado: 1–2 semanas

FASE D — 3D (pipeline opcional, para futuro)
Objetivo
- Planejar e permitir expansão para 3D: meshes, vertex/index buffers, camera 3D, iluminação básica.

Entregáveis (planejamento + PoC)
- Abstração de Mesh/Material separada do Sprite.
- Camera3D e transformação 4x4.
- Pipeline WGSL para mesh rendering e depth testing.

Passos detalhados
1. Modelar `Mesh`, `Material` e formatos de vertex.
2. Reaproveitar abstrações de renderer (RenderQueue, Pipeline) para mesh passes.
3. Criar PoC que carrega um mesh simples (quad/box) e renderiza com depth test.

Critérios de aceitação
- PoC que desenha meshes com depth e transforma com Camera3D.

Tempo estimado: 2–4 semanas (após estabilidade 2D/2.5D)

NODES INSTANCIÁVEIS, PREFABS E ASSETS
Objetivo
- Permitir criação, serialização, prefab e instanciamento eficiente de nodes (pooling/instancing).

Passos detalhados
1. Definir formato de prefab (toml/ron/json) com lista de components e valores iniciais.
2. Implementar `PrefabLoader` que cria nodes por descrição e registra no `NodeManager`.
3. Implementar pooling para objects de alto churn (bullets, partículas) e instancing para render.
4. Serialização/Deserialização de cena para salvar/carregar.

Critérios de aceitação
- Exemplo que carrega um prefab para spawnar N instâncias rapidamente e reproduz comportamentos.

INTEGRAÇÃO, TESTES E VALIDAÇÃO
Checklist de qualidade
- Unit tests: transform math, AABB collision, prefab loader.
- Smoke test: `game` example que roda headless e com cliente gráfico.
- Performance tests: render batches, instancing, memory usage.

Ferramentas e convenções
- Código e estilo: manter padrões idiomáticos Rust; documentar API pública do core.
- CI: `cargo build --workspace` + `cargo test` em PRs.

Próximos passos imediatos (ordem recomendada)
1. Implementar `wgpu` state e exemplo clear screen.
2. Implementar `Node`, `Transform2D`, `Sprite` e render de um quad.
3. Implementar `Velocity` e update loop.
4. Implementar colisão AABB.
5. Iterar em batching/atlas e hot-reload.

Comandos úteis
```fish
# Build
cargo build --workspace

# Executar exemplo (após criar o binário `game`)
cargo run -p game
```

Notas finais
- Divida as tarefas em PRs pequenos e testáveis; cada PR deve ter um pequeno exemplo ou teste que demonstre o comportamento.


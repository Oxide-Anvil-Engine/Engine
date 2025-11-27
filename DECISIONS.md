
## Oxide Anvil — DECISIONS (Documento estratégico)

Data: 2025-11-20 (atualizado)

Este arquivo documenta as decisões de alto nível, princípios e restrições que guiarão o desenvolvimento da engine. Ele contém escolhas que devem se manter estáveis ao longo do tempo; metas e passos concretos ficam em `OBJETIVOS.md`.

1) Visão e princípios
- Visão: construir uma engine modular, performática e de iteração rápida — foco em qualidade, mas com liberdade para experimentação.
- Princípios: segurança (Rust), performance (medir antes de otimizar), volatilidade (hot-reload), modularidade (backends isolados), clareza de API.

2) Linguagem e ecossistema
- Linguagem do core: **Rust** (por segurança e ecossistema).
- Ferramentas iniciais: `wgpu` (render), `naga` (shaders), `tokio` (async/network), `rkyv` (serialização de alto desempenho).

3) Arquitetura de render e abstração
- Ter uma camada de abstração `Renderer` (trait) que exponha operações de alto nível: criar recursos, criar pipelines, submeter comandos, apresentar, resize.
- Inicialmente centralizar o desenvolvimento em `wgpu` (reduz custo de manutenção). Projetar a arquitetura para permitir backends nativos posteriormente.
- Manter representação interna neutra de recursos (formats, vertex layouts, descriptors) para facilitar porting entre backends.

4) Shaders e toolchain
- Linguagem preferida na fase inicial: **WGSL** (wgpu/WebGPU).
- Estratégia: gerar artefatos por plataforma no build (WGSL / SPIR-V / MSL / HLSL conforme necessário). Ferramentas: `naga`, `spirv-cross`, `shaderc`, `DXC`.
- Automatizar instalações via scripts ou Docker; se baixar bins em runtime, exigir verificação por checksum/assinatura.

5) Plugins, hot-reload e extensibilidade
- Preferir **WASM** para scripts/plugins (sandbox e multi-linguagem). Oferecer C ABI mínima para integrações nativas quando estritamente necessário.
- Habilitar hot-reload de shaders e módulos de script para iteração rápida.

6) Multiplayer — visão geral
- Arquitetura alvo: server-authoritative, snapshotting, interest management. Transportes: UDP/QUIC; serialização: `rkyv`.
- Prioridade inicial: infra headless para treinar AIs; integrar rede completa após estabilizar a simulação local.

7) Riscos e governança
- Licenças de ferramentas (SPIRV-Cross, DXC, etc.) e redistribuição de binários.
- Interoperabilidade entre drivers/APIs pode ser frágil; evitar misturar dispositivos nativos sem plano de compartilhamento sólido.

8) Plataforma: manejo de APIs gráficas e estratégia multi-API
Esta seção resume recomendações práticas por plataforma e uma estratégia para permitir que projetos utilizem múltiplas APIs gráficas quando necessário.

Principais plataformas e APIs recomendadas
- Windows: **DirectX 12** (integração nativa) ou **Vulkan** para portabilidade; usar DXC e Vulkan SDK.
- Linux: **Vulkan** (preferência); SPIR-V toolchain recomendada.
- macOS / iOS: **Metal** nativo; MoltenVK pode ser usado como camada compatível (com limitações).
- Web: **WebGPU / WGSL** (via wgpu/dawn).
- Android: **Vulkan** (nativo).

Estratégias práticas para múltiplas APIs
- Camada de abstração: defina `Renderer` e mantenha todo código do jogo dependente apenas desse contrato.
- Backend primário + compositor offscreen: escolha um backend para apresentar (ex.: wgpu). Outros backends podem renderizar offscreen; o compositor (no backend primário) realiza composição. Bom para prototipagem.
- Compartilhamento avançado (zero-copy): usar extensões de memória externa (VK_KHR_external_memory, semaphores exportáveis) entre APIs — alto desempenho, alta complexidade e requisita testes intensivos por driver.
- wgpu-first: escrever a maior parte do código em wgpu, reaproveitando seus backends internos; ideal para reduzir trabalho de interoperabilidade.

Shader e recurso pipeline
- Use WGSL como forma canônica para wgpu; gere SPIR-V para Vulkan/DX quando necessário; converta para MSL com SPIRV-Cross para Metal.
- Pré-compile shaders por plataforma durante o build para determinismo; permita hot-reload em desenvolvimento.

Operações práticas e testes
- Defina formatos neutros para recursos (metadados de textures/materials) para conversão automática no load.
- Execute smoke-tests por backend (clear, draw quad, upload texture) e integre em CI com runners por plataforma.

Tradeoffs e recomendação inicial
- Comece com `wgpu` como backend principal e invista em uma pipeline de shaders que produza artefatos por plataforma. Quando houver necessidade de otimizações extremas, projete a migração para backends nativos via a camada `Renderer` e, se necessário, adicione compositor offscreen ou interop de memória.

---

DECISIONS.md deve permanecer enxuto e orientador; detalhes operacionais e passos ficam em `OBJETIVOS.md`.


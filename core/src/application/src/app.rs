use crate::frames::FramesManager;
use crate::graphics::backend::backend_trait::Backend;
use crate::graphics::backend::wgpu::backend_api::{BackendOptionsWGPU, BackendWGPU};
use crate::graphics::render::renderer::Renderer;
use std::sync::Arc;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::graphics::resources::registers::manager::ResourcesManager;

pub struct App {
    event_loop: EventLoop<()>,
    window: Arc<Window>,
    backend: Box<BackendWGPU>, // deixar generico depois
    renderer: Renderer,
    frames_manager: FramesManager,
    register_manager: ResourcesManager,
}

impl App {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().expect("event loop");
        let window = WindowBuilder::new()
            .with_title("My Winit Window")
            .build(&event_loop)
            .expect("window");
        let window = Arc::new(window);
        let size = window.inner_size();

        // Se BackendWGPU::new aceitar Arc<Window>
        let backend = BackendWGPU::new(
            Arc::clone(&window), // sem & aqui
            (size.width, size.height),
            BackendOptionsWGPU::default(),
        );

        Self {
            event_loop,
            window,
            backend: Box::new(backend),
            renderer: Renderer::new(),
            frames: FramesManager::default(),
        }
    }

    pub fn run(mut self) {
        // winit 0.29: controla o fluxo pelo EventLoop
        self.event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = self; // mover estado para o closure
        app.event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(sz) => app.backend.resize(sz.width, sz.height),
                    WindowEvent::RedrawRequested => {
                        app.renderer.render(
                            &mut app.backend,
                            &mut app.register_manager,
                        );
                    },
                    _ => {}
                },
                Event::AboutToWait => {
                    app.window.request_redraw();
                },

                _ => {}
            }
        });
    }
}

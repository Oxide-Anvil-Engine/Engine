use std::time::Duration;
use std::time::Instant;

use graphics::backend::backend_trait::Backend;
use graphics::backend::wgpu::backend_api::{BackendOptionsWGPU, BackendWGPU};
use graphics::render::renderer::Renderer;
use graphics::resources::registers::manager::ResourcesManager;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::window::Window;
use winit::window::WindowAttributes;

use winit::error::EventLoopError;

use crate::frames::FrameManager;
use crate::frames::FrameManagerDesc;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::runtime::Tokio;

pub struct App {
    window: Option<Arc<Window>>,
    backend: Option<Box<BackendWGPU>>, // deixar generico depois
    renderer: Renderer,
    frame_manager: FrameManager,
    register_manager: ResourcesManager,
    tokio: Tokio,
    desc: AppDesc,
}

#[derive(Clone)]
pub struct AppDesc {
    control_flow: ControlFlow,
    frame_manager_desc: FrameManagerDesc,
    window_desc: WindowAttributes,
}

impl Default for AppDesc {
    fn default() -> Self {
        const TARGET_FPS: f32 = 60.0;
        Self {
            control_flow: ControlFlow::WaitUntil(
                Instant::now() + Duration::from_millis((1000.0 / TARGET_FPS) as u64),
            ),
            frame_manager_desc: FrameManagerDesc::default(),
            window_desc: WindowAttributes::default()
                .with_title("Oxide Anvil Engine"),
        }
    }
}

impl AppDesc {
    pub fn new(control_flow: ControlFlow, frame_manager_desc: FrameManagerDesc, window_desc: WindowAttributes) -> Self {
        Self {
            control_flow,
            frame_manager_desc,
            window_desc,
        }
    }
}

impl ApplicationHandler for App {
    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
        if self.desc.control_flow != ControlFlow::Poll {
            event_loop.set_control_flow(match self.frame_manager.next_frame_control_flow() {
                Some(dt) => ControlFlow::WaitUntil(Instant::now() + dt),
                _ => ControlFlow::Poll,
            });
        }
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(sz) => self.backend.as_mut().unwrap().resize(sz.width, sz.height),
            WindowEvent::RedrawRequested => {
                self.renderer.render(
                    self.backend.as_mut().unwrap().as_mut(),
                    &self.register_manager,
                );
                self.frame_manager.register_frame();
            }
            _ => {}
        }
    }
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop
                .create_window(self.desc.window_desc.clone())
                .expect("create window"),
        ));
        self.backend = Some(Box::new(BackendWGPU::new(
            Arc::clone(self.window.as_ref().unwrap()),
            BackendOptionsWGPU::default(),
        )));
        self.frame_manager.last_frame_instant = Some(Instant::now());
    }
}

impl App {
    pub fn new(desc: AppDesc) -> Self {
        let frame_manager = FrameManager::new(desc.clone().frame_manager_desc);
        frame_manager.run();
        let register_manager = ResourcesManager::new();
        let tokio = Tokio::new();
        Self {
            window: None,
            backend: None,
            renderer: Renderer::new(),
            frame_manager,
            register_manager,
            tokio,
            desc,
        }
    }

    pub fn run(mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new().expect("create event loop");
        event_loop.set_control_flow(self.desc.control_flow);
        event_loop.run_app(&mut self)
    }
}

impl Default for App {
    fn default() -> Self {
        let desc = AppDesc::default();
        Self::new(desc)
    }
}

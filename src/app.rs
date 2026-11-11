//! Application loop: window lifecycle, input, fixed script/physics update, rendering.
use crate::{ecs, renderer::Renderer, script::ScriptHost};
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, TouchPhase, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    script: Option<ScriptHost>,
    last: Instant,
}

impl App {
    pub fn new() -> Self {
        Self { window: None, renderer: None, script: None, last: Instant::now() }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.window.is_none() {
            let attrs = Window::default_attributes().with_title("NovaForge");
            self.window = Some(Arc::new(el.create_window(attrs).expect("create window")));
        }
        let window = self.window.clone().unwrap();
        let renderer = pollster::block_on(Renderer::new(window.clone()));
        let (w, h) = renderer.size();
        ecs::with(|c| c.screen = (w as f32, h as f32));
        self.renderer = Some(renderer);
        if self.script.is_none() {
            let host = ScriptHost::new(include_str!("../assets/main.py"));
            host.call_start();
            self.script = Some(host);
        }
        self.last = Instant::now();
        window.request_redraw();
    }

    fn suspended(&mut self, _el: &ActiveEventLoop) {
        // Android destroys the native surface on suspend.
        self.renderer = None;
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(s) => {
                if let Some(r) = self.renderer.as_mut() {
                    r.resize(s.width, s.height);
                }
                ecs::with(|c| c.screen = (s.width as f32, s.height as f32));
            }
            WindowEvent::CursorMoved { position, .. } => {
                ecs::with(|c| {
                    c.input.x = position.x as f32;
                    c.input.y = position.y as f32;
                });
            }
            WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => {
                ecs::with(|c| c.input.down = state == ElementState::Pressed);
            }
            WindowEvent::Touch(t) => {
                ecs::with(|c| {
                    c.input.x = t.location.x as f32;
                    c.input.y = t.location.y as f32;
                    c.input.down = matches!(t.phase, TouchPhase::Started | TouchPhase::Moved);
                });
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = (now - self.last).as_secs_f32().min(0.1);
                self.last = now;
                ecs::with(|c| c.time += dt);
                if let Some(s) = &self.script {
                    s.call_update(dt);
                }
                ecs::with(|c| ecs::physics_system(c, dt));
                if let Some(r) = self.renderer.as_mut() {
                    r.render();
                }
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            _ => {}
        }
    }
}

pub fn run(event_loop: EventLoop<()>) {
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new();
    event_loop.run_app(&mut app).expect("event loop");
}

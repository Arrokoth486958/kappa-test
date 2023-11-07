use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::wgpu::RenderInstance;

pub struct Application<'a> {
    render_instance: RenderInstance,
    window: &'a Window,
}

impl<'a> Application<'a> {
    pub fn new(window: &'a Window) -> Self {
        let render_instance = RenderInstance::new(window);

        Application {
            window,
            render_instance,
        }
    }

    pub fn on_loop(
        &mut self,
        event: Event<()>,
        elwt: &winit::event_loop::EventLoopWindowTarget<()>,
    ) {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                if window_id == self.window.id() {
                    match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }
                        WindowEvent::Resized(size) => {
                            #[cfg(target_os = "macos")]
                            self.window.request_redraw();
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {}
                        WindowEvent::KeyboardInput { .. } => {}
                        WindowEvent::RedrawRequested => {}
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

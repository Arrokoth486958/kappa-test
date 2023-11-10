use winit::{
    event::{Event, WindowEvent},
    window::Window,
};

use crate::wgpu::RenderInstance;

#[allow(dead_code)]
pub struct Application<'a> {
    render_instance: RenderInstance,
    window: &'a Window,
}

impl<'a> Application<'a> {
    pub fn new(window: &'a Window) -> Result<Self, Box<dyn std::error::Error>> {
        let render_instance = pollster::block_on(RenderInstance::new(window))?;

        Ok(Application {
            window,
            render_instance,
        })
    }

    pub fn on_loop(
        &mut self,
        event: Event<()>,
        elwt: &winit::event_loop::EventLoopWindowTarget<()>,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
                        WindowEvent::Resized(_size) => {
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
        Ok(())
    }
}

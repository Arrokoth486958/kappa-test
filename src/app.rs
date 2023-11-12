use winit::{
    event::{Event, WindowEvent},
    window::Window,
};

use crate::render::RenderInstance;

#[allow(dead_code)]
pub struct Application<'a> {
    render_instance: RenderInstance,
    window: &'a Window,
}

impl<'a> Application<'a> {
    pub fn new(window: &'a Window) -> Result<Self, Box<dyn std::error::Error>> {
        let render_instance = pollster::block_on(RenderInstance::new(window))?;

        Ok(Application {
            render_instance,
            window,
        })
    }

    // 芝士主循环事件
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
                        WindowEvent::Resized(size) => {
                            // 为了修复窗口在重新放大后被别的窗口挡住
                            self.window.focus_window();

                            self.render_instance.resize(*size);
                            #[cfg(target_os = "macos")]
                            self.window.request_redraw();
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {}
                        WindowEvent::KeyboardInput { .. } => {}
                        WindowEvent::RedrawRequested => {
                            self.render_instance.render()?;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

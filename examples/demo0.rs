use kappa::app::Application;
use winit::{window::{WindowBuilder, WindowButtons}, dpi::LogicalSize, event_loop::EventLoop};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut window_builder = WindowBuilder::new()
        .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE)
        .with_inner_size(LogicalSize::new(600, 400))
        .with_min_inner_size(LogicalSize::new(600, 400))
        .with_title("Kappa")
        .with_visible(false);
    let window = window_builder.build(&event_loop).unwrap();
    let mut app = Application::new(&window);

    window.set_visible(true);
    event_loop.run(move |event, elwt: &winit::event_loop::EventLoopWindowTarget<()>| {
        // TODO: 你好
        app.on_loop(event, elwt);
    }).unwrap();
}
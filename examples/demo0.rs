use kappa::app::Application;
use log::info;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{WindowBuilder, WindowButtons},
};

fn main() {
    kappa::log::init().expect("Could not initialize logging system!");

    info!("Initializing Kappa...");

    let event_loop = EventLoop::new().unwrap();
    let window_builder = WindowBuilder::new()
        .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE)
        .with_inner_size(LogicalSize::new(600, 400))
        .with_min_inner_size(LogicalSize::new(600, 400))
        .with_title("Kappa")
        .with_visible(false);
    let window = window_builder.build(&event_loop).unwrap();
    let mut app = Application::new(&window).unwrap();

    window.set_visible(true);
    event_loop
        .run(
            move |event, elwt: &winit::event_loop::EventLoopWindowTarget<()>| {
                // TODO: 你好
                let result = app.on_loop(event, elwt);
                if result.is_err() {
                    log::error!("{}", result.err().unwrap().to_string());
                }
            },
        )
        .unwrap();
}

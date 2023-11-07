use wgpu::{Instance, InstanceDescriptor, Backends};
use winit::window::Window;

pub struct RenderInstance {
}

impl RenderInstance {
    pub fn new(window: &Window) -> Self {
        let wgpu_instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { wgpu_instance.create_surface(window) }.expect("Could not create a Surface!");

        RenderInstance {
        }
    }
}
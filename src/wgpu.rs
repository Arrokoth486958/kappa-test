use wgpu::{Backends, Instance, InstanceDescriptor, RequestAdapterOptions, Backend};
use winit::window::Window;

pub struct RenderInstance {
    wgpu_instance: Instance,
}

impl RenderInstance {
    pub async fn new(window: &Window) -> Self {
        let wgpu_instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            // backends: Backends::DX11 | Backends::DX12 | Backends::METAL | Backends::VULKAN | Backends::GL,
            dx12_shader_compiler: Default::default(),
        });

        let surface =
            unsafe { wgpu_instance.create_surface(window) }.expect("Could not create a Surface!");
        
        // TODO: 支持Backend优先级
        let adapter = wgpu_instance.request_adapter(&RequestAdapterOptions {
            compatible_surface: Some(&surface),
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
        }).await.unwrap();

        // log::info!("1");

        // let 

        RenderInstance {
            wgpu_instance,
        }
    }
}

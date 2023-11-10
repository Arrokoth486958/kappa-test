use wgpu::{
    Backends, CompositeAlphaMode, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits,
    RequestAdapterOptions, SurfaceConfiguration, TextureUsages,
};
use winit::window::Window;

use crate::error::KappaError;

#[allow(dead_code)]
pub struct RenderInstance {
    wgpu_instance: Instance,
}

// 一些可能用得上的东西：https://jinleili.github.io/learn-wgpu-zh
impl RenderInstance {
    pub async fn new(window: &Window) -> Result<Self, Box<dyn std::error::Error>> {
        let size = window.inner_size();

        let wgpu_instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface =
            unsafe { wgpu_instance.create_surface(window) }.expect("Could not create a Surface!");

        // TODO: 支持Backend优先级
        let adapter = wgpu_instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(KappaError::new("Could not create an adapter"))?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features: Features::default(),
                    limits: Limits::downlevel_defaults(),
                },
                None,
            )
            .await
            ?;

        let caps = surface.get_capabilities(&adapter);

        // TODO: 更好，更强，更壮（指选择alpha channel
        let alpha_channel = if caps
            .alpha_modes
            .contains(&CompositeAlphaMode::PostMultiplied)
        {
            CompositeAlphaMode::PostMultiplied
        } else {
            caps.alpha_modes[0]
        };

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: alpha_channel,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Ok(RenderInstance { wgpu_instance })
    }
}

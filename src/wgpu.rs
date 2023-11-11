use wgpu::{
    Adapter, Backends, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration,
    TextureUsages, RenderPassDescriptor, RenderPassColorAttachment, Operations, LoadOp, Color,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{error::KappaError, renderer::RenderSystem};

#[allow(dead_code)]
pub struct RenderInstance {
    pub(crate) size: PhysicalSize<u32>,
    pub(crate) wgpu_instance: Instance,
    pub(crate) surface: Surface,
    pub(crate) adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) config: SurfaceConfiguration,
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
            .await?;

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
            present_mode: wgpu::PresentMode::Immediate,
            alpha_mode: alpha_channel,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Ok(RenderInstance {
            size,
            wgpu_instance,
            adapter,
            config,
            device,
            queue,
            surface,
        })
    }

    pub fn reconfigure(&mut self) {
        self.surface.configure(&self.device, &self.config);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.size = size;
            self.config.width = size.width;
            self.config.height = size.height;
            self.reconfigure();
        }
    }

    pub fn render(&mut self, render_system: &mut RenderSystem) -> Result<(), Box<dyn std::error::Error>> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_system.render(&mut render_pass)?;
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }
}

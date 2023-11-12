use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use wgpu::{
    include_wgsl, util::DeviceExt, Adapter, Backends, BlendState, Buffer, BufferAddress, Color,
    ColorTargetState, ColorWrites, CompositeAlphaMode, Device, DeviceDescriptor, Face, Features,
    FragmentState, Instance, InstanceDescriptor, Limits, LoadOp, MultisampleState, Operations,
    PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, Surface,
    SurfaceConfiguration, TextureFormat, TextureUsages, VertexAttribute, VertexBufferLayout,
    VertexState,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{cache, error::KappaError};

static mut VERTEX_BUFFERS: Vec<Buffer> = Vec::new();
static mut INDEX_BUFFERS: Vec<Buffer> = Vec::new();

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4, 2 => Float32x2];

    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct RenderObject {
    vertex_addr: usize,
    index_addr: usize,
}

impl RenderObject {
    pub fn new(vertex: Vec<Vertex>, index: Vec<u16>) -> RenderObject {
        RenderObject {
            vertex_addr: cache::alloc_vertex(vertex),
            index_addr: cache::alloc_index(index),
        }
    }
}

#[allow(dead_code)]
pub struct RenderInstance {
    pub(crate) size: PhysicalSize<u32>,
    pub(crate) wgpu_instance: Instance,
    pub(crate) surface: Surface,
    pub(crate) adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) config: SurfaceConfiguration,
    pub(crate) pipelines: HashMap<String, RenderPipeline>,
    pub(crate) render_objects: Vec<RenderObject>,
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
            unsafe { wgpu_instance.create_surface(window) }?;

        // TODO: 支持Backend优先级
        let adapter = wgpu_instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(KappaError::new("Could not create an adapter"))?;

        for i in wgpu_instance.enumerate_adapters(Backends::all()) {
            println!("{:?}", i.get_info());
        }

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
        // 很后悔没有早点在macOS上测试
        // 现在好了，渲染不出来了
        // 好玄乎，又可以跑了
        let alpha_channel = if caps
            .alpha_modes
            .contains(&CompositeAlphaMode::PostMultiplied)
        {
            CompositeAlphaMode::PostMultiplied
        } else {
            caps.alpha_modes[0]
        };
        let alpha_channel: CompositeAlphaMode = caps.alpha_modes[0];
        println!("{:?}", caps.alpha_modes);
        // let alpha_channel: CompositeAlphaMode = CompositeAlphaMode::Opaque;

        // TODO: 这玩意咋筛？
        let supported_formats = caps.formats.clone();
        println!("{:?}", supported_formats);
        let format = if supported_formats.contains(&TextureFormat::Bgra8Unorm) {
            TextureFormat::Bgra8Unorm
        } else {
            supported_formats[0]
        };
        // let format = supported_formats[0];

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: alpha_channel,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let mut pipelines = HashMap::new();
        // let shader = instance.device.create_shader_module(ShaderModuleDescriptor {
        //     label: Some("Position Color Shader"),
        //     source: include_wgsl!("position_color.wgsl"),
        // });
        let shader = device.create_shader_module(include_wgsl!("position_color.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Position Color Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            depth_stencil: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: ColorWrites::all(),
                })],
            }),
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                // 剔除部分
                // TODO：有意思的东西
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(Face::Front),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        pipelines.insert("position_color".into(), pipeline);

        let mut render_objects = Vec::new();

        // TODO: Debug
        render_objects.push(RenderObject::new(
            vec![
                // 左下
                Vertex {
                    position: [0.25, 0.25, 0.0],
                    color: [1.0, 0.0, 1.0, 0.5],
                    tex_coords: [0.0, 0.0],
                },
                // 右下
                Vertex {
                    position: [0.75, 0.25, 0.0],
                    color: [0.0, 0.0, 1.0, 0.5],
                    tex_coords: [0.0, 0.0],
                },
                // 右上
                Vertex {
                    position: [0.75, 0.75, 0.0],
                    color: [0.0, 1.0, 0.0, 0.5],
                    tex_coords: [0.0, 0.0],
                },
                // 左下
                Vertex {
                    position: [0.25, 0.75, 0.0],
                    color: [1.0, 0.0, 0.0, 0.5],
                    tex_coords: [0.0, 0.0],
                },
            ],
            vec![0, 1, 2, 2, 3, 0],
        ));

        Ok(RenderInstance {
            size,
            wgpu_instance,
            adapter,
            config,
            device,
            queue,
            surface,
            pipelines,
            render_objects,
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

    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
            for obj in &self.render_objects {
                unsafe {
                    let vertex_buffer =
                        self.device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: None,
                                contents: bytemuck::cast_slice(cache::get_vertex(obj.vertex_addr)),
                                usage: wgpu::BufferUsages::VERTEX,
                            });

                    let index_buffer =
                        self.device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: None,
                                contents: bytemuck::cast_slice(cache::get_index(obj.index_addr)),
                                usage: wgpu::BufferUsages::INDEX,
                            });

                    // 所有权转移
                    VERTEX_BUFFERS.push(vertex_buffer);
                    INDEX_BUFFERS.push(index_buffer);

                    render_pass.set_pipeline(
                        self.pipelines
                            .get("position_color".into())
                            .ok_or(KappaError::new("Unable to set Render Pipeline!"))?,
                    );
                    render_pass.set_vertex_buffer(
                        0,
                        VERTEX_BUFFERS
                            .last()
                            .ok_or(KappaError::new("Unable to set Vertex Buffer!"))?
                            .slice(..),
                    );
                    render_pass.set_index_buffer(
                        INDEX_BUFFERS
                            .last()
                            .ok_or(KappaError::new("Unable to get last index buffer!"))?
                            .slice(..),
                        wgpu::IndexFormat::Uint16,
                    );

                    render_pass.draw_indexed(
                        0..cache::get_index(obj.index_addr).len() as u32,
                        0,
                        0..1,
                    );
                }
            }
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        unsafe {
            for i in &VERTEX_BUFFERS {
                i.destroy();
            }
            VERTEX_BUFFERS.clear();
            for i in &INDEX_BUFFERS {
                i.destroy();
            }
            INDEX_BUFFERS.clear();
        }

        Ok(())
    }
}

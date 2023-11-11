use std::{collections::HashMap, sync::Arc};

use bytemuck::{Pod, Zeroable};
use wgpu::{RenderPipeline, RenderPipelineDescriptor, PrimitiveState, PipelineLayoutDescriptor, Face, VertexState, VertexAttribute, VertexBufferLayout, BufferAddress, FragmentState, ColorTargetState, BlendState, ColorWrites, MultisampleState, include_wgsl, RenderPass};

use crate::wgpu::RenderInstance;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2];

    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[allow(dead_code)]
pub struct RenderSystem {
    pipelines: HashMap<String, Arc<RenderPipeline>>,
}

impl RenderSystem {
    pub fn new(instance: &RenderInstance) -> Result<Self, Box<dyn std::error::Error>> {
        // 渲染管线
        let pipelines = HashMap::new();

        // let shader = instance.device.create_shader_module(ShaderModuleDescriptor {
        //     label: Some("Position Color Shader"),
        //     source: include_wgsl!("position_color.wgsl"),
        // });
        let shader = instance.device.create_shader_module(include_wgsl!("position_color.wgsl"));

        let pipeline_layout = instance.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Position Color Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let _pipeline = instance.device.create_render_pipeline(&RenderPipelineDescriptor {
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
                    format: instance.config.format,
                    blend: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: ColorWrites::all(),
                })],
            }),
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                // 剔除部分
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(Face::Back),
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

        Ok(RenderSystem {
            pipelines,
        })
    }

    pub fn render(&mut self, _render_pass: &mut RenderPass) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

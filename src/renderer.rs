use std::{collections::HashMap, sync::Arc};

use bytemuck::{Pod, Zeroable};
use wgpu::{RenderPipeline, RenderPipelineDescriptor, PrimitiveState, PipelineLayoutDescriptor, Face, VertexState, VertexAttribute, VertexBufferLayout, BufferAddress, FragmentState, ColorTargetState, BlendState, ColorWrites, MultisampleState, include_wgsl, RenderPass, Buffer, Device, util::DeviceExt};

use crate::{wgpu::RenderInstance, cache};

#[allow(dead_code)]
pub struct RenderSystem {
}

impl RenderSystem {
    pub fn new(instance: &RenderInstance) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(RenderSystem {
        })
    }

    pub fn render(&mut self, render_pass: &mut RenderPass, instance: &mut RenderInstance) -> Result<(), Box<dyn std::error::Error>> {

        Ok(())
    }
}

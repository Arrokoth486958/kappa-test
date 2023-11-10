use crate::wgpu::RenderInstance;

#[allow(dead_code)]
pub struct RenderSystem<'a> {
    instance: &'a RenderInstance,
}

impl<'a> RenderSystem<'a> {
    pub fn new(instance: &'a RenderInstance) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(RenderSystem {
            instance,
        })
    }
}

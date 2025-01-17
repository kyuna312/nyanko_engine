use std::sync::Arc;
use vulkano::device::Device;
use vulkano::shader::ShaderModule;

pub struct Shader {
    module: Arc<ShaderModule>,
}

impl Shader {
    pub fn new(device: Arc<Device>, code: &[u8]) -> Result<Self, vulkano::VulkanError> {
        let module = unsafe { ShaderModule::from_bytes(device, code)? };
        Ok(Shader { module })
    }

    pub fn vertex(device: Arc<Device>, code: &[u8]) -> Result<Self, vulkano::VulkanError> {
        Self::new(device, code)
    }

    pub fn fragment(device: Arc<Device>, code: &[u8]) -> Result<Self, vulkano::VulkanError> {
        Self::new(device, code)
    }

    pub fn module(&self) -> &Arc<ShaderModule> {
        &self.module
    }
}

// Example vertex shader
pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
            #version 450
            layout(location = 0) in vec2 position;
            layout(location = 0) out vec2 uv;
            void main() {
                uv = position * 0.5 + 0.5;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "
    }
}

// Example fragment shader
pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
            #version 450
            layout(location = 0) in vec2 uv;
            layout(location = 0) out vec4 f_color;
            void main() {
                f_color = vec4(uv, 0.0, 1.0);
            }
        "
    }
}

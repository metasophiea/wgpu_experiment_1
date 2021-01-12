#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
}
impl Vertex {
    pub fn new(position:[f32; 2]) -> Vertex {
        Vertex {
            position,
        }
    }
}
impl Vertex {
    pub fn get_x(&self) -> f32 { self.position[0] }
    pub fn get_y(&self) -> f32 { self.position[1] }
}
impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> { 
        wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float2,
                },
            ],
        }
    }
}
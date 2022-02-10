
pub const U16_SIZE: wgpu::BufferAddress = std::mem::size_of::<u16>() as wgpu::BufferAddress;
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32;2]
}

impl Vertex {
    pub const SIZE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;
    pub const DESC: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: Self::SIZE,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x2,
        ]
    };
}
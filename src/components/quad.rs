use std::mem::size_of;

use glam::{Vec2, vec2, UVec2, uvec2};
use wgpu::{Buffer, Device, util::{BufferInitDescriptor, DeviceExt}, BufferUsages, BufferDescriptor, ShaderModule, BindGroupLayout, BindGroupLayoutDescriptor, ShaderStages, BindingType, TextureSampleType, BindGroupLayoutEntry, SamplerBindingType, BindGroup, BindGroupDescriptor, BindingResource, BindGroupEntry, BufferBindingType, BufferSize};

use super::transform::Transform;

pub struct Quad {
    pub size: UVec2,
    pub vertices: [f32; 12],

    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,
    pub mesh_buffer: Option<Buffer>,
}

impl Quad {
    pub const VERTEX_SIZE: u64 = size_of::<[f32;20]>() as u64;
    pub const INDEX_SIZE: u64 = size_of::<[u16; 6]>() as u64;

    pub const INDICES: [u16; 6] = [
        0, 1, 2,
        0, 2, 3
    ];

    pub const UVS: [f32; 8] = [
        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0,
        0.0, 0.0
    ];

    pub fn get_vertex_data(&self) -> [f32; 20] {
        let mut data: [f32; 20] = [0.0; 20];
        for i in 0..4 {
            let mut data_offset = i * 5;
            let vert_offset = i * 3;
            let uv_offset = i * 2;
            data[data_offset..(data_offset + 3)].clone_from_slice(&self.vertices[vert_offset..(vert_offset + 3)]);
            data_offset += 3;
            data[data_offset..(data_offset + 2)].clone_from_slice(&Self::UVS[uv_offset..(uv_offset + 2)]);
        }

        data
    }

    pub fn get_index_data(&self) -> &[u8] {
        bytemuck::cast_slice(&Quad::INDICES)
    }

    pub fn build_buffers(&mut self, device: &Device) {
        let mut data: [f32; 20] = [0.0; 20];
        for i in 0..4 {
            let mut data_offset = i * 5;
            let vert_offset = i * 3;
            let uv_offset = i * 2;
            data[data_offset..(data_offset + 3)].clone_from_slice(&self.vertices[vert_offset..(vert_offset + 3)]);
            data_offset += 3;
            data[data_offset..(data_offset + 2)].clone_from_slice(&Self::UVS[uv_offset..(uv_offset + 2)]);
        }

        let data = bytemuck::bytes_of(&data);
        self.vertex_buffer = Some(device.create_buffer_init(
            &BufferInitDescriptor {
            label: Some("mesh_vertex_buffer"),
            contents: data,
            usage: BufferUsages::VERTEX
        }));

        self.index_buffer = Some(device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("mesh_index_buffer"),
                contents: bytemuck::cast_slice(&Quad::INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        ));
    }
}

impl From<UVec2> for Quad {
    fn from(size: UVec2) -> Self {
        let extent_x = size.x as f32 * 0.5;
        let extent_y = size.y as f32 * 0.5;

        let vertices = [
            -extent_x, -extent_y, 0.0,
            extent_x, -extent_y, 0.0,
            extent_x, extent_y, 0.0,
            -extent_x, extent_y, 0.0
        ];

        Self {
            size,
            vertices,
            vertex_buffer: None,
            index_buffer: None,
            mesh_buffer: None,
        }
    }
}

impl Default for Quad {
    fn default() -> Self {
        Self::from(uvec2(100, 100))
    }
}
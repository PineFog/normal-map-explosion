use glam::{Vec3, Quat, Mat4};
use wgpu::{Buffer, Device, util::{BufferInitDescriptor, DeviceExt}, BufferUsages, BufferDescriptor, ShaderModule, BindGroupLayout, BindGroupLayoutDescriptor, ShaderStages, BindingType, TextureSampleType, BindGroupLayoutEntry, SamplerBindingType, BindGroup, BindGroupDescriptor, BindingResource, BindGroupEntry, BufferBindingType, BufferSize};

pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub transform_buffer: Option<Buffer>,
    pub bind_group: Option<BindGroup>
}

impl Transform {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self::from_translation(Vec3::new(x, y, z))
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Transform {
            translation,
            ..Default::default()
        }
    }

    pub const fn identity() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            transform_buffer: None,
            bind_group: None
        }
    }

    pub fn get_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    pub fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ],
            label: Some("transform_bind_group_layout")
        })
    }

    pub fn build_buffers(&mut self, device: &Device, layout: &BindGroupLayout) {
        self.transform_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: Some("mesh_buffer"),
            contents: bytemuck::cast_slice(&[self.get_matrix()]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        }));

        self.bind_group = Some(device.create_bind_group(
            &BindGroupDescriptor {
                layout: layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: self.transform_buffer.as_ref().unwrap().as_entire_binding()
                    },
                ],
                label: Some("mesh_bind_group")
            }
        ));
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}
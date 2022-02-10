use crevice::std140::{AsStd140, Std140};
use glam::*;
use glam::{Vec3};
use wgpu::{Buffer, Device, util::{BufferInitDescriptor, DeviceExt}, BufferUsages, BindGroupLayout, BindGroupLayoutDescriptor, ShaderStages, BindingType, BindGroupLayoutEntry, BindGroup, BufferBindingType, BufferSize};


use super::{transform::Transform, Quad};

#[derive(AsStd140)]
pub struct GPUBaseLight {
    pub color: glam::Vec3,
    pub ambient_intensity: f32,
    pub diffuse_intensity: f32
}

impl Default for GPUBaseLight {
    fn default() -> GPUBaseLight {
        GPUBaseLight {
            color: vec3(1.0, 1.0, 1.0),
            ambient_intensity: 0.0,
            diffuse_intensity: 5.0
        }
    }
}

#[derive(AsStd140)]
pub struct GPUAttenuation {
    pub constant: f32,
    pub linear: f32,
    pub exp: f32,
}

impl Default for GPUAttenuation {
    fn default() -> GPUAttenuation {
        GPUAttenuation {
            constant: 0.0,
            linear: 0.2,
            exp: 0.2
        }
    }
}

#[derive(AsStd140)]
pub struct EyePosition {
    pub eye_position: Vec3
}

#[derive(AsStd140)]
pub struct ScreenSize {
    pub screen_size: Vec2
}

#[derive(AsStd140)]
pub struct GPUPointLight {
    pub base_light: GPUBaseLight,
    pub position: glam::Vec3,
    pub atten: GPUAttenuation
}

impl Default for GPUPointLight {
    fn default() -> Self {
        Self {
            base_light: GPUBaseLight::default(),
            position: vec3(0.0, 0.0, 0.0),
            atten: GPUAttenuation::default(),
        }
    }
}

impl GPUPointLight {
    pub fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(
        &BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new(Self::std140_size_static() as u64)
                        },
                        count: None
                    }
                ],
                label: Some("point_light_bind_group_layout")
            }
        )
    }
}

#[derive(AsStd140)]
pub struct GPUSpecularAttributes {
    power: f32,
    intensity: f32
}

impl Default for GPUSpecularAttributes {
    fn default() -> Self {
        Self {
            power: 0.0,
            intensity: 0.0
        }
    }
}

impl GPUSpecularAttributes {
    pub fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(
        &BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new(Self::std140_size_static() as u64)
                        },
                        count: None
                    }
                ],
                label: Some("specular_attributes_group_layout")
            }
        )
    }
}

pub struct PointLight {
    pub transform: Transform,
    pub mesh: Quad,
    pub gpu_light: GPUPointLight,
    pub light_buffer: Option<Buffer>,
    pub bind_group: Option<BindGroup>,
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            mesh: Quad::default(),
            gpu_light: GPUPointLight::default(),
            light_buffer: None,
            bind_group: None
        }
    }
}

impl PointLight {
    pub fn build_buffers(&mut self, device: &Device) {

        let light_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: Some("light_buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: self.gpu_light.as_std140().as_bytes()
        }));
        self.light_buffer = light_buffer;
    }
}
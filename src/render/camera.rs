use glam::{Mat4, Quat};
use wgpu::{Device, ShaderStages, BufferBindingType, BindingType, BindGroupLayoutEntry, BindGroupDescriptor, BindGroupEntry, util::{BufferInitDescriptor, DeviceExt}, BufferUsages, BindGroupLayout, BindGroup, Buffer, Queue};

use crate::components::transform::Transform;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    //view_proj: [[f32; 4]; 4],
    // view_proj: Mat4
}

pub struct Camera {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
    pub transform: Transform,
    pub zoom: f32,

    pub proj_matrix: Mat4,
    pub view_matrix: Mat4,
    pub view_proj_matrix: Mat4,

    pub bind_group_layout: Option<BindGroupLayout>,
    pub bind_group: Option<BindGroup>,
    pub camera_buffer: Option<Buffer>,

    dirty: bool
}

pub fn degress_to_radians(degrees: f32) -> f32 {
    degrees * 0.0174533
}

impl Camera {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let near = 500.0;
        let far = -500.0;
        let t = Transform::from_xyz(0.0, 0.0, 0.1);
        let zoom = 1.0;

        let view_matrix = Mat4::IDENTITY;
        let proj_matrix = Mat4::orthographic_rh(
            left * zoom, 
            right * zoom, 
            bottom * zoom, 
            top * zoom, 
            near, 
            far
        );
        let view_proj_matrix = proj_matrix * view_matrix;

        Self {
            left,
            right,
            bottom,
            top,
            near,
            far,
            transform: t,
            proj_matrix,
            view_matrix,
            view_proj_matrix,
            bind_group_layout: None,
            bind_group: None,
            camera_buffer: None,
            zoom,
            dirty: true
        }
    }

    pub fn recalculate_view_matrix(&mut self) {
        let transform = Mat4::from_scale_rotation_translation(self.transform.scale, self.transform.rotation, self.transform.translation);
        self.view_matrix = transform.inverse();
        self.view_proj_matrix = self.proj_matrix * self.view_matrix;
    }

    pub fn zoom_in(&mut self, amount: f32) {
        self.zoom += amount;
        // let aspect = 800.0 / 600.0;

        println!("{}", (self.left * self.zoom) / (self.bottom * self.zoom));
        
        self.proj_matrix = Mat4::orthographic_rh(
            self.left * self.zoom, 
            self.right * self.zoom, 
            self.bottom * self.zoom, 
            self.top * self.zoom, 
            self.near, 
            self.far
        );
        self.view_proj_matrix = self.proj_matrix * self.view_matrix;
    }

    pub fn set_projection(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        let near = 500.0;
        let far = -500.0;
        //let t = &self.transform;
        self.proj_matrix = Mat4::orthographic_rh(
            left * self.zoom, 
            right * self.zoom, 
            bottom * self.zoom, 
            top * self.zoom, 
            near, 
            far
        );
        self.view_proj_matrix = self.proj_matrix * self.view_matrix;
    }

    pub fn build_buffers(&mut self, device: &Device) {
        // let mut camera_uniform = CameraUniform::new();

        //let test :[[f32; 4]; 4] = self.view_proj_matrix
        //let test = bytemuck::cast_slice(&[self.view_proj_matrix]);

        self.camera_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: Some("camera_buffer"),
            contents: bytemuck::cast_slice(&[self.view_proj_matrix]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        }));

        // self.bind_group_layout = Some(device.create_bind_group_layout(
        //     &wgpu::BindGroupLayoutDescriptor {
        //         entries: &[
        //             BindGroupLayoutEntry {
        //                 binding: 0,
        //                 visibility: ShaderStages::VERTEX,
        //                 ty: BindingType::Buffer {
        //                     ty: BufferBindingType::Uniform,
        //                     has_dynamic_offset: false,
        //                     min_binding_size: None
        //                 },
        //                 count: None
        //             }
        //         ],
        //         label: Some("camera_bind_group_layout")
        //     }
        // ));

        // self.bind_group = Some(device.create_bind_group(
        //     &BindGroupDescriptor {
        //         layout: self.bind_group_layout.as_ref().unwrap(),
        //         entries: &[
        //             BindGroupEntry {
        //                 binding: 0,
        //                 resource: self.camera_buffer.as_ref().unwrap().as_entire_binding()
        //             },
        //         ],
        //         label: Some("camera_bind_group")
        //     }
        // ));
    }
}
use std::{collections::HashMap};
use wgpu::{Device,BindGroup, BindGroupDescriptor, BindingResource, BindGroupEntry};
use crate::render::texture::Texture;
use super::{transform::Transform, Quad};

pub struct Sprite {
    pub transform: Transform,
    pub mesh: Quad,
    pub bind_group: Option<BindGroup>,
}

impl Sprite {
    pub fn build_buffers(&mut self, device: &Device, textures: &HashMap<&str, Texture>) {
        // let mut data: [f32; 20] = [0.0; 20];
        // for i in 0..4 {
        //     let mut data_offset = i * 5;
        //     let vert_offset = i * 3;
        //     let uv_offset = i * 2;
        //     data[data_offset..(data_offset + 3)].clone_from_slice(&self.mesh.vertices[vert_offset..(vert_offset + 3)]);
        //     data_offset += 3;
        //     data[data_offset..(data_offset + 2)].clone_from_slice(&Quad::UVS[uv_offset..(uv_offset + 2)]);
        // }

        // let layout = sprite_manager.bind_group_layouts.get(self.texture_name.as_str()).unwrap();

        // let texture = textures.get(self.texture_name.as_str()).unwrap();
        // let normal_texture = textures.get(format!("{}_normal",self.texture_name).as_str()).unwrap();
        // self.bind_group = Some(device.create_bind_group(
        //     &BindGroupDescriptor {
        //         layout: layout,
        //         entries: &[
        //             BindGroupEntry {
        //                 binding: 0,
        //                 resource: BindingResource::TextureView(&texture.view)
        //             },
        //             BindGroupEntry {
        //                 binding: 1,
        //                 resource: BindingResource::TextureView(&normal_texture.view)
        //             },
        //             BindGroupEntry {
        //                 binding: 2,
        //                 resource: BindingResource::Sampler(&texture.sampler)
        //             },
        //             BindGroupEntry {
        //                 binding: 3,
        //                 resource: BindingResource::Sampler(&normal_texture.sampler)
        //             }
        //         ],
        //         label: Some("sprite_texture_bind_group")
        //     }
        // ));
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            mesh: Quad::default(),
            bind_group: None,
        }
    }
}
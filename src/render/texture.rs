use std::num::NonZeroU32;

use glam::{Vec2, vec2, UVec2, uvec2};
use image::GenericImageView;
use wgpu::{
    BindGroupLayout,
    BindGroup, Device, Queue
};

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: UVec2
}

impl Texture {
    pub fn from_bytes(
        device: &Device,
        queue: &Queue,
        bytes: &[u8],
        label: &str,
        is_normal_map: bool
    ) -> Self {
        let img = image::load_from_memory(bytes).unwrap();
        Self::from_image(device, queue, &img, label, is_normal_map)
    }

    pub fn from_image(
        device: &Device,
        queue: &Queue,
        img: &image::DynamicImage,
        label: &str,
        is_normal_map: bool,
    ) -> Self {
        let dimensions = img.dimensions();
        let rgba = img.to_rgba8();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: if is_normal_map {
                wgpu::TextureFormat::Rgba8Unorm
            } else {
                wgpu::TextureFormat::Rgba8UnormSrgb
            },
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * dimensions.0),
                rows_per_image: NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            size: uvec2(size.width, size.height),
            texture,
            view,
            sampler
        }

    }
}
use wgpu::{Instance, Surface, SurfaceConfiguration, Device, Queue, BindGroupLayout, ShaderModule, RenderPipeline, Buffer, BindGroup};
use winit::{event_loop::EventLoop, window::Window};

use crate::{render::{Texture, Camera}, components::{Sprite, PointLight}};

pub struct State {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Option<Window>,
    pub instance: Option<Instance>,
    pub surface: Option<Surface>,
    pub surface_config: Option<SurfaceConfiguration>,
    pub device: Option<Device>,
    pub queue: Option<Queue>,
    pub albedo_texture: Option<Texture>,
    pub normal_texture: Option<Texture>,
    pub camera: Option<Camera>,
    pub sprite: Option<Sprite>,
    pub sprite_buffer: Option<Buffer>,
    pub light: Option<PointLight>,
    pub light_buffer: Option<Buffer>,
    pub layouts: Option<[BindGroupLayout; 3]>,
    pub bind_groups: Option<[BindGroup; 3]>,
    pub pipeline: Option<RenderPipeline>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            event_loop: None,
            window: None,
            instance: None,
            surface: None,
            surface_config: None,
            device: None,
            queue: None,
            albedo_texture: None,
            normal_texture: None,
            camera: None,
            sprite: None,
            light: None,
            layouts: None,
            bind_groups: None,
            pipeline: None,
            sprite_buffer: None,
            light_buffer: None
        }
    }
}
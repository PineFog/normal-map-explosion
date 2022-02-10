mod state;
mod render;
mod components;

use std::borrow::Cow;

use components::{Transform, Sprite, Quad, GPUPointLight, PointLight, GPUBaseLight};
use crevice::std140::{AsStd140, Std140};
use glam::{vec3, Mat4};
use render::{Camera, Texture, Vertex};
use state::State;
use wgpu::{Instance, SurfaceConfiguration, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStages, BindingType, BufferBindingType, BufferSize, TextureSampleType, SamplerBindingType, ShaderModuleDescriptor, ShaderSource, RenderPipelineDescriptor, VertexState, BlendComponent, util::{BufferInitDescriptor, DeviceExt}, BufferUsages, BufferAddress, BufferDescriptor, BindGroupDescriptor, BindGroupEntry, BindingResource, BufferBinding, RenderPassColorAttachment, RenderPassDescriptor, IndexFormat};
use winit::{window::WindowBuilder, dpi::PhysicalSize, event_loop::{ControlFlow, EventLoop}, event::{Event, WindowEvent}};

const INITIAL_SCREEN_SIZE: PhysicalSize<u32> = PhysicalSize::new(1280, 720);

fn main() {
    let mut state = State::default();

    futures::executor::block_on(init_window(&mut state));
    load_textures(&mut state);
    create_camera(&mut state);
    create_sprite(&mut state);
    create_light(&mut state);
    create_layouts(&mut state);
    create_forward_pass(&mut state);
    create_buffers(&mut state);
    write_buffers(&mut state);
    create_bind_groups(&mut state);
    run_loop(state);
}

fn write_buffers(state: &mut State) {
    let queue = state.queue.as_ref().unwrap();
    let camera = state.camera.as_mut().unwrap();
    let sprite = state.sprite.as_ref().unwrap();
    let light = state.light.as_ref().unwrap();

    queue.write_buffer(camera.camera_buffer.as_ref().unwrap(), 0, bytemuck::cast_slice(&[camera.view_proj_matrix]));
    queue.write_buffer(state.sprite_buffer.as_ref().unwrap(), 0, bytemuck::cast_slice(&[sprite.transform.get_matrix()]));
    queue.write_buffer(state.light_buffer.as_ref().unwrap(), 0, light.gpu_light.as_std140().as_bytes());
}

fn create_buffers(state: &mut State) {
    let device = state.device.as_ref().unwrap();
    let camera = state.camera.as_mut().unwrap();
    let sprite = state.sprite.as_mut().unwrap();
    let uniform_alignment = device.limits().min_uniform_buffer_offset_alignment as BufferAddress;

    camera.build_buffers(device);
    state.sprite_buffer = Some(device.create_buffer(&BufferDescriptor {
        label: Some("sprite_transform_buffer"),
        size: uniform_alignment as u64,
        mapped_at_creation: false,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
    }));
    sprite.mesh.build_buffers(device);

    state.light_buffer = Some(device.create_buffer(&BufferDescriptor {
        label: Some("gpu_light_buffer"),
        size: uniform_alignment as u64,
        mapped_at_creation: false,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
    }));
}

fn create_forward_pass(state: &mut State) {
    let device = state.device.as_ref().unwrap();
    let layouts = state.layouts.as_ref().unwrap();
    let bind_group_layouts = &[&layouts[0], &layouts[1], &layouts[2]];

    let vert_shader = device.create_shader_module(&ShaderModuleDescriptor {
        label: Some("vert_shader"),
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("../res/vertex.wgsl")))
    });

    let frag_shader = device.create_shader_module(&ShaderModuleDescriptor {
        label: Some("frag_shader"),
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("../res/frag.wgsl")))
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts,
        push_constant_ranges: &[]
    });

    state.pipeline = Some(device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("point_light_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: &vert_shader,
            entry_point: "vs",
            buffers: &[Vertex::DESC]
        },
        fragment: Some(wgpu::FragmentState {
            module: &frag_shader,
            entry_point: "fs",
            targets: &[wgpu::ColorTargetState {
                format: state.surface_config.as_ref().unwrap().format,
                blend: Some(wgpu::BlendState {
                    alpha: BlendComponent::REPLACE,
                    color: BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: true,
        },
        multiview: None,
        depth_stencil: None,
    }));
}

fn create_bind_groups(state: &mut State) {
    let device = state.device.as_ref().unwrap();
    let layouts = state.layouts.as_ref().unwrap();
    let camera = state.camera.as_mut().unwrap();

    let pass_bind_group = device.create_bind_group(
        &BindGroupDescriptor {
            layout: &layouts[0],
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: camera.camera_buffer.as_ref().unwrap().as_entire_binding()
                },
            ],
            label: Some("pass_bind_group")
        }
    );

    let material_bind_group = device.create_bind_group(
        &BindGroupDescriptor {
            layout: &layouts[1],
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&state.albedo_texture.as_ref().unwrap().view)
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&state.normal_texture.as_ref().unwrap().view)
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&state.albedo_texture.as_ref().unwrap().sampler)
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::Sampler(&state.normal_texture.as_ref().unwrap().sampler)
                }
            ],
            label: Some("material_bind_group")
        }
    );

    let object_bind_group = device.create_bind_group(
        &BindGroupDescriptor {
            layout: &layouts[2],
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer( BufferBinding {
                        buffer: state.sprite_buffer.as_ref().unwrap(),
                        offset: 0,
                        size: BufferSize::new(Mat4::std140_size_static() as u64)
                    })
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer( BufferBinding {
                        buffer: state.light_buffer.as_ref().unwrap(),
                        offset: 0,
                        size: BufferSize::new(GPUPointLight::std140_size_static() as u64)
                    })
                }
            ],
            label: Some("object_bind_group")
        }
    );
    state.bind_groups = Some([pass_bind_group, material_bind_group, object_bind_group]);
}

fn create_layouts(state: &mut State) {
    let device = state.device.as_ref().unwrap();
    let group_0 = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("pass_layout"),
        entries: &[
            BindGroupLayoutEntry {  // camera view projection
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(Mat4::std140_size_static() as u64)
                },
                count: None
            }
        ]
    });

    let group_1 = device.create_bind_group_layout(
        &BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {  // albedo texture
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: TextureSampleType::Float { filterable: false}
                    },
                    count: None
                },
                BindGroupLayoutEntry {  // normal texture
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: TextureSampleType::Float { filterable: false}
                    },
                    count: None
                },
                BindGroupLayoutEntry {  // albedo sampler
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
                BindGroupLayoutEntry {  // normal sampler
                    binding: 3,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
            label: Some("material_layout")
        }
    );

    let group_2 = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("object_layout"),
        entries: &[
            BindGroupLayoutEntry {  // sprite mesh
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: BufferSize::new(Mat4::std140_size_static() as u64)
                },
                count: None
            },
            BindGroupLayoutEntry {  // light values
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: BufferSize::new(GPUPointLight::std140_size_static() as u64)
                },
                count: None
            }
        ]
    });
    state.layouts = Some([group_0, group_1, group_2]);
}

fn create_light(state: &mut State) {
    let light = PointLight{transform: Transform::from_translation(vec3(0.0, 0.0,0.5)), 
        gpu_light: GPUPointLight {base_light: GPUBaseLight {diffuse_intensity: 50.0, ..Default::default()}, ..Default::default()}, ..Default::default()};
    state.light = Some(light);
}

fn create_sprite(state: &mut State) {
    let texture = state.albedo_texture.as_ref().unwrap();
    let mut transform = Transform::default();
    transform.scale = vec3(0.1, 0.1, 1.1);
    transform.translation = vec3(0.0, 0.0, 0.0);
    let sprite = Sprite {
        mesh: Quad::from(texture.size),
        transform,
        ..Default::default()
    };
    state.sprite = Some(sprite);
}

fn load_textures(state: &mut State) {
    let device = state.device.as_ref().unwrap();
    let queue = state.queue.as_ref().unwrap();
    let albedo_bytes = include_bytes!("../res/bump_diffuse.png");
    let normal_bytes = include_bytes!("../res/bump_normal.png");
    let albedo_texture = Texture::from_bytes(device, queue, albedo_bytes, "bump_diffuse", false);
    let normal_texture = Texture::from_bytes(device, queue, normal_bytes, "bump_normal", false);
    state.albedo_texture = Some(albedo_texture);
    state.normal_texture = Some(normal_texture);
}

fn create_camera(state: &mut State) {
    let magnitude = INITIAL_SCREEN_SIZE.width as f32 + INITIAL_SCREEN_SIZE.height as f32;
    let norm_x = INITIAL_SCREEN_SIZE.width as f32 / magnitude;
    let norm_y = INITIAL_SCREEN_SIZE.height as f32 / magnitude;

    let mut camera = Camera::new(-norm_x, norm_x, -norm_y, norm_y);
    camera.zoom_in(100.0);
    camera.recalculate_view_matrix();
    state.camera = Some(camera);
}

async fn init_window(state: &mut State) {
    let event_loop = winit::event_loop::EventLoop::with_user_event();
    
    
    let window = WindowBuilder::new()
        .with_visible(false)
        .with_title("Draw Something")
        .build(&event_loop)
        .unwrap();
    if window.fullscreen().is_none() {
        window.set_inner_size(INITIAL_SCREEN_SIZE);
    }
    window.set_cursor_visible(false);

    let instance = Instance::new(wgpu::Backends::all());
    let surface = unsafe {instance.create_surface(&window)};
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface)
    }).await
    .unwrap();

    let (device, queue) = adapter
    .request_device(&wgpu::DeviceDescriptor {
        label: None,
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::downlevel_webgl2_defaults()
            .using_resolution(adapter.limits())
    }, None)
    .await
    .expect("Failed to create device");

    let config = SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(&adapter).unwrap(),
        width: INITIAL_SCREEN_SIZE.width,
        height: INITIAL_SCREEN_SIZE.height,
        present_mode: wgpu::PresentMode::Immediate
    };
    surface.configure(&device, &config);
    window.set_visible(true);

    state.event_loop = Some(event_loop);
    state.instance = Some(instance);
    state.surface = Some(surface);
    state.surface_config = Some(config);
    state.device = Some(device);
    state.queue = Some(queue);
    state.window = Some(window);
}

fn run_loop(mut state: State) {
    state.event_loop.take().unwrap().run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        let device = state.device.as_ref().unwrap();
        let queue = state.queue.as_ref().unwrap();
        let surface_config = state.surface_config.as_mut().unwrap();
        let surface = state.surface.as_ref().unwrap();
        let bind_groups = state.bind_groups.as_ref().unwrap();
        let sprite = state.sprite.as_ref().unwrap();

        match event {
            Event::NewEvents(_) => {
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,..} => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                ref event,
                window_id
            } => {
                match event {
                    WindowEvent::Resized(size) => {
                        surface_config.width = size.width;
                        surface_config.height = size.height;
                        surface.configure(device, surface_config);
                    },
                    _ => {}
                }
            },
            Event::MainEventsCleared => {
                let output = state.surface.as_ref().unwrap().get_current_texture().unwrap();
                let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                let render_pass_descriptor = RenderPassDescriptor {
                    label: Some("render_pass_descriptor"),
                    color_attachments: &[
                        RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0,}),
                                store: true,
                            },
                        },
                    ],
                    depth_stencil_attachment: None
                };
                let mut pass = encoder.begin_render_pass(&render_pass_descriptor);
                pass.set_pipeline(state.pipeline.as_ref().unwrap());
                pass.set_bind_group(0, &bind_groups[0], &[]);  // view/camera
                pass.set_bind_group(1, &bind_groups[1], &[]);  // material/textures
                pass.set_bind_group(2, &bind_groups[2], &[0, 0]);  // object/sprite/light
                pass.set_vertex_buffer(0, sprite.mesh.vertex_buffer.as_ref().unwrap().slice(..));
                pass.set_index_buffer( sprite.mesh.index_buffer.as_ref().unwrap().slice(..), IndexFormat::Uint16);
                pass.draw_indexed(0..Quad::INDICES.len() as u32, 0, 0..1);

                drop(pass);
                queue.submit(core::iter::once(encoder.finish()));
                output.present();
            },
            _ => {}
        }
    });
}
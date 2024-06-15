use std::{mem, sync::Arc};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use web_time::{Duration, Instant};

use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use cgmath::{ElementWise, Vector2};
use wgpu::{
    include_wgsl, util::{BufferInitDescriptor, DeviceExt}, vertex_attr_array, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites, FragmentState, InstanceDescriptor, InstanceFlags, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, RenderPipelineDescriptor, ShaderStages, Surface, SurfaceTarget, VertexBufferLayout, VertexState
};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Camera {
    center: Vector2<f32>,
    size: Vector2<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Vertex(Vector2<f32>);
impl Vertex {
    pub fn layout() -> VertexBufferLayout<'static> {
        let attributes = &vertex_attr_array![0 => Float32x2];

        VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes,
        }
    }
}

pub struct MandelbrotRenderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    current_window_size: (u32, u32),

    vertex_buffer: wgpu::Buffer,

    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    render_pipeline: wgpu::RenderPipeline,
}

impl MandelbrotRenderer {
    #[cfg(target_arch = "wasm32")]
    pub async fn new_from_canvas(
        size: (u32, u32),
        canvas: web_sys::HtmlCanvasElement,
        vsync: bool,
    ) -> Result<Self> {
        Self::new(size, SurfaceTarget::Canvas(canvas), vsync).await
    }

    pub async fn new(
        size: (u32, u32),
        surface: impl Into<wgpu::SurfaceTarget<'static>>,
        vsync: bool,
    ) -> Result<Self> {
        let instance = wgpu::Instance::new(InstanceDescriptor {
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(surface).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::default(),
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: if vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            },
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let camera_buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: mem::size_of::<Camera>() as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            mapped_at_creation: false, 
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &camera_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_shader = device.create_shader_module(include_wgsl!("shaders/vert.wgsl"));
        let fragment_shader = device.create_shader_module(include_wgsl!("shaders/frag.wgsl"));

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(QUAD_VERTS),
            usage: BufferUsages::VERTEX,
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &vertex_shader,
                entry_point: "main",
                buffers: &[Vertex::layout()],
                compilation_options: PipelineCompilationOptions::default(),
            },
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &fragment_shader,
                entry_point: "main",
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: surface_config.format,
                    blend: None,
                    write_mask: ColorWrites::all(),
                })],
            }),
            multiview: None,
        });

        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            current_window_size: size,
            vertex_buffer,
            camera_buffer,
            camera_bind_group,
            render_pipeline,
            surface_config,
        })
    }

    pub fn render(&mut self, camera_center: (f32, f32), camera_size: (f32, f32)) {
        // Update camera
        let camera = Camera {
            center: Vector2::from(camera_center),
            size: Vector2::from(camera_size),
        };
        self.queue
            .write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera]));

        // Get render target texture
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Render to texture
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..(QUAD_VERTS.len() as u32), 0..1);
        }

        // Submit and present
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    pub fn resize(&mut self, (width, height): (u32, u32)) {
        if width > 0 && height > 0 {
            self.current_window_size = (width, height);
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }
}

const QUAD_VERTS: &[Vertex] = &[
    Vertex(Vector2::new(-1.0, 1.0)),  // top left
    Vertex(Vector2::new(-1.0, -1.0)), // bot left
    Vertex(Vector2::new(1.0, 1.0)),   // top right
    Vertex(Vector2::new(1.0, 1.0)),   // top right
    Vertex(Vector2::new(-1.0, -1.0)), // bot left
    Vertex(Vector2::new(1.0, -1.0)),  // bot right
];

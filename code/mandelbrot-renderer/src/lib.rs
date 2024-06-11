use std::{mem, sync::Arc};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use web_time::{Duration, Instant};

use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use cgmath::{ElementWise, Vector2};
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    vertex_attr_array, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BufferUsages, ColorTargetState, ColorWrites, FragmentState,
    InstanceDescriptor, InstanceFlags, MultisampleState, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PrimitiveState, RenderPipelineDescriptor, ShaderStages, Surface,
    SurfaceTarget, VertexBufferLayout, VertexState,
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

pub struct MandelbrotExplorer {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    current_window_size: (u32, u32),

    vertex_buffer: wgpu::Buffer,

    camera: Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    current_size_exponent: f32,

    render_pipeline: wgpu::RenderPipeline,

    is_mouse_held_down: bool,
    previous_mouse_position: Option<Vector2<f32>>,
}

impl MandelbrotExplorer {
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

        let camera = Camera {
            center: Vector2::new(0.0, 0.0),
            size: Vector2::new(1.0, 1.0),
        };

        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[camera]),
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
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
            camera,
            camera_buffer,
            camera_bind_group,
            current_size_exponent: 0.0,
            render_pipeline,
            is_mouse_held_down: false,
            previous_mouse_position: None,
            // last_fps_measurement_time: Instant::now(),
            // current_fps_measurement_frame_counter: 0,
            surface_config,
        })
    }

    pub fn render(&mut self) {
        self.queue
            .write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera]));

        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
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

        // submit will accept anything that implements IntoIter
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

    /// Mouse press and release event
    pub fn on_mouse_click(&mut self, is_down: bool) {
        self.is_mouse_held_down = is_down;
    }

    /// `delta` is lines scrolled
    pub fn on_mouse_wheel(&mut self, delta: f32) {
        // TODO zoom in where mouse currently is
        self.current_size_exponent = (self.current_size_exponent - delta).max(-80.0).min(10.0);

        let size = 1.1_f32.powf(self.current_size_exponent);
        self.camera.size = Vector2::new(size, size);
    }

    /// `new_position` is in pixels
    pub fn on_mouse_move(&mut self, new_position: (f32, f32)) {
        let position = Vector2::from(new_position);
        if self.is_mouse_held_down {
            if let Some(previous_mouse_position) = self.previous_mouse_position {
                let delta = previous_mouse_position - position;
                let window_size = Vector2::new(
                    self.current_window_size.0 as f32,
                    self.current_window_size.1 as f32,
                );
                let mut delta_normalized = delta.div_element_wise(window_size);

                delta_normalized = delta_normalized.mul_element_wise(Vector2::new(2.0, -2.0));

                self.camera.center += delta_normalized.mul_element_wise(self.camera.size);
            }
        }
        self.previous_mouse_position = Some(position);
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

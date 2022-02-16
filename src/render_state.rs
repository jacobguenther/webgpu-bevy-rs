// File: render_state.rs

use crate::primitives::*;
use crate::vertex::*;
use crate::mesh::*;
use crate::camera::*;
use crate::texture::*;

use winit::{
	window::Window,
	event::*,
};

use wgpu::util::DeviceExt;

use bevy::prelude::Component;

#[derive(Component)]
pub struct RenderState {
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	size: winit::dpi::PhysicalSize<u32>,

	depth_texture: Texture,

	render_pipeline: wgpu::RenderPipeline,

	quad: Mesh,
	sphere: Mesh,

	camera: Camera,
	camera_uniform: CameraUniform,
	camera_buffer: wgpu::Buffer,
	camera_bind_group: wgpu::BindGroup,
	camera_controller: CameraController,
}
impl RenderState {
	pub async fn new(window: &Window) -> Self {
		let size = window.inner_size();

		let instance = wgpu::Instance::new(wgpu::Backends::all());
		let surface = unsafe {
			instance.create_surface(window)
		};
		let adapter = instance.request_adapter(
			&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			},
		).await.unwrap();

		let (device, queue) = adapter.request_device(
			&wgpu::DeviceDescriptor {
				features: wgpu::Features::empty(),
				limits: wgpu::Limits::default(),
				label: None,
			},
			None,
		).await.unwrap();

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface.get_preferred_format(&adapter).unwrap(),
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Fifo,
		};
		surface.configure(&device, &config);

		// Camera setup
		let camera = {
			let projection = {
				let fovy = 45.0;
				let znear = 0.1;
				let zfar = 1000.0;
				Projection::new(size.width, size.height, fovy, znear, zfar)
			};
			let eye = cgmath::Point3::new(0.0, 2.0, 2.0);
			let target = cgmath::Point3::new(0.0, 0.0, 0.0);
			Camera::new(eye, target, &projection)
		};
		let mut camera_uniform = CameraUniform::new();
		camera_uniform.load_view_proj(&camera);
		let camera_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Camera Buffer"),
				contents: bytemuck::cast_slice(&[camera_uniform]),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			}
		);
		let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}
			],
			label: Some("camera_bind_group_layout"),
		});
		let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &camera_bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: camera_buffer.as_entire_binding(),
				}
			],
			label: Some("camera_bind_group"),
		});		
		let camera_controller = {
			let speed = 1.0;
			CameraController::new(speed)
		};

		let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");


		let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
			label: Some("Shader"),
			source: wgpu::ShaderSource::Wgsl(include_str!("../assets/shaders/shader.wgsl").into()),
		});

		let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Render Pipeline Layout"),
			bind_group_layouts: &[
				&camera_bind_group_layout
			],
			push_constant_ranges: &[],
		});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Render Pipeline"),
			layout: Some(&render_pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[
					Vertex::desc(),
				],
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[wgpu::ColorTargetState {
					format: config.format,
					blend: Some(wgpu::BlendState::REPLACE),
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
			depth_stencil: Some(wgpu::DepthStencilState {
				format: Texture::DEPTH_FORMAT,
				depth_write_enabled: true,
				depth_compare: wgpu::CompareFunction::Less,
				stencil: wgpu::StencilState::default(),
				bias: wgpu::DepthBiasState::default(),
			}),
			multisample: wgpu::MultisampleState {
				count: 1,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
			multiview: None,
		});

		let quad = Mesh::new(&device, Some("quad"), &QUAD_VERTICES, &QUAD_INDICES, 6);
		let mut sphere_generator = MeshGenerator::default();
		sphere_generator.uv_sphere(0.2, 16, 16);
		let sphere = Mesh::new(&device, Some("sphere"), &sphere_generator.vertices, &sphere_generator.indices, sphere_generator.indices.len() as u32);
		RenderState {
			surface,
			device,
			queue,
			config,
			size,

			depth_texture,

			render_pipeline,

			quad,
			sphere,

			camera,
			camera_uniform,
			camera_buffer,
			camera_bind_group,
			camera_controller,
		}
	}
	pub fn resize(&mut self, width: u32, height: u32) {
		if width > 0 && height > 0 {
			self.size = winit::dpi::PhysicalSize::new(width, height);
			self.config.width = width;
			self.config.height = height;
			self.depth_texture = Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
			self.surface.configure(&self.device, &self.config);
		}
	}
	pub fn input(&mut self, _event: &WindowEvent) -> bool {
		false
	}
	pub fn update(&mut self) {
		
	}
	pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
		let output = self.surface.get_current_texture()?;
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("Render Encoder"),
		});

		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0.1,
							g: 0.2,
							b: 0.3,
							a: 1.0,
						}),
						store: true,
					},
				}],
				depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
					view: &self.depth_texture.view,
					depth_ops: Some(wgpu::Operations {
						load: wgpu::LoadOp::Clear(1.0),
						store: true,
					}),
					stencil_ops: None,
				}),
			});

			render_pass.set_pipeline(&self.render_pipeline);

			render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

			render_pass.set_vertex_buffer(0, self.quad.vertex_buffer.slice(..));
			render_pass.set_index_buffer(self.quad.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
			render_pass.draw_indexed(0..self.quad.num_indices, 0, 0..1);


			render_pass.set_vertex_buffer(0, self.sphere.vertex_buffer.slice(..));
			render_pass.set_index_buffer(self.sphere.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
			render_pass.draw_indexed(0..self.sphere.num_indices, 0, 0..1);
		}
	
		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();
	
		Ok(())
	}
}
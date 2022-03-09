// File: render_state.rs

use crate::camera::*;
use crate::mesh::*;
use crate::texture::*;
use crate::vertex::*;

use winit::window::Window;

use wgpu::DeviceType;

pub struct RenderState {
	pub surface: wgpu::Surface,
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
	pub config: wgpu::SurfaceConfiguration,
	pub size: winit::dpi::PhysicalSize<u32>,

	depth_texture: Texture,

	// diffuse_texture: Texture,
	diffuse_bind_group: TextureBindGroup,

	render_pipeline: wgpu::RenderPipeline,
}
impl RenderState {
	pub async fn new(window: &Window) -> Self {
		let size = window.inner_size();

		assert!(size.width != 0);
		assert!(size.height != 0);

		let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
		let surface = unsafe { instance.create_surface(window) };

		println!("* * * ADAPTERS * * *");
		let adapter = instance
			.enumerate_adapters(wgpu::Backends::all())
			.fold(None, |acc, adapter| {
				println!("{:?}", adapter);
				println!("{:?}\n", adapter.get_info());

				// If the adapter doesn't support the surface continue
				let supports_surface = surface.get_preferred_format(&adapter).is_some();
				if !supports_surface {
					return acc;
				}
				match acc {
					// Anything is better than nothing
					None => Some(adapter),
					Some(current) => {
						// Prefere discrete GPUs
						match (
							current.get_info().device_type,
							adapter.get_info().device_type,
						) {
							(_, DeviceType::DiscreteGpu) => Some(adapter),
							_ => Some(current),
						}
					}
				}
			})
			.unwrap();

		println!("SELECTED ADAPTER");
		println!("{:?}", adapter);
		println!("{:?}\n", adapter.get_info());

		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					features: wgpu::Features::empty(),
					limits: wgpu::Limits::default(),
					label: None,
				},
				None,
			)
			.await
			.unwrap();

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface.get_preferred_format(&adapter).unwrap(),
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Fifo,
		};
		surface.configure(&device, &config);

		// create depth texture
		let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

		// create diffuse texture
		let diffuse_bytes = include_bytes!("../assets/images/earth.png");
		let diffuse_texture =
			crate::texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "earth_png")
				.unwrap();
		let diffuse_bind_group =
			TextureBindGroup::new(&device, Some("diffuse_bind_group"), &diffuse_texture);

		// create render pipeline
		let camera_bind_group_layout = CameraBindGroup::create_layout(&device);

		let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
			label: Some("shader"),
			source: wgpu::ShaderSource::Wgsl(include_str!("../assets/shaders/shader.wgsl").into()),
		});

		let render_pipeline_layout =
			device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: Some("render_pipeline_layout"),
				bind_group_layouts: &[&camera_bind_group_layout, &diffuse_bind_group.layout],
				push_constant_ranges: &[],
			});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("render_pipeline"),
			layout: Some(&render_pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[Vertex::desc()],
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

		RenderState {
			surface,
			device,
			queue,
			config,
			size,

			depth_texture,

			// diffuse_texture,
			diffuse_bind_group,

			render_pipeline,
		}
	}
	pub fn resize(&mut self, width: u32, height: u32) {
		if width > 0 && height > 0 {
			self.size = winit::dpi::PhysicalSize::new(width, height);
			self.config.width = width;
			self.config.height = height;
			self.depth_texture =
				Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
			self.surface.configure(&self.device, &self.config);
		}
	}
	pub fn render<'a>(
		&self,
		camera_bind_group: &CameraBindGroup,
		meshes: impl Iterator<Item = &'a Mesh>,
	) -> Result<(), wgpu::SurfaceError> {
		let output = self.surface.get_current_texture()?;
		let view = output
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		let mut render_encoder =
			self.device
				.create_command_encoder(&wgpu::CommandEncoderDescriptor {
					label: Some("render_encoder"),
				});

		{
			let mut render_pass = render_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("render_pass"),
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

			render_pass.set_bind_group(0, &camera_bind_group.bind_group, &[]);
			render_pass.set_bind_group(1, &self.diffuse_bind_group.bind_group, &[]);

			for mesh in meshes {
				render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
				render_pass
					.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
				render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
			}
		}

		self.queue.submit(std::iter::once(render_encoder.finish()));
		output.present();

		Ok(())
	}
}

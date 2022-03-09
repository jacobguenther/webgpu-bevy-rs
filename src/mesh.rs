// File: mesh.rs

use crate::vertex::*;

use wgpu::util::DeviceExt;

use bevy::ecs::component::Component;

use core::f32::consts::PI;
use std::time::Instant;

#[derive(Component)]
pub struct ShouldDraw;

#[derive(Component)]
pub struct Mesh {
	pub vertex_buffer_label: Option<String>,
	pub vertex_buffer: wgpu::Buffer,

	pub index_buffer_label: Option<String>,
	pub index_buffer: wgpu::Buffer,
	pub num_indices: u32,
}
impl Mesh {
	pub fn new(
		device: &wgpu::Device,
		label: Option<&str>,
		vertices: &[Vertex],
		indices: &[u16],
		num_indices: u32,
	) -> Self {
		let (vertex_buffer_label, index_buffer_label) = match label {
			Some(l) => (
				Some(format!("{} vertex buffer label", l)),
				Some(format!("{} index buffer label", l)),
			),
			None => (None, None),
		};

		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: vertex_buffer_label.as_ref().map(|v| v.as_str()),
			contents: bytemuck::cast_slice(vertices),
			usage: wgpu::BufferUsages::VERTEX,
		});

		let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: index_buffer_label.as_ref().map(|v| v.as_str()),
			contents: bytemuck::cast_slice(indices),
			usage: wgpu::BufferUsages::INDEX,
		});

		Self {
			vertex_buffer_label,
			vertex_buffer,
			index_buffer_label,
			index_buffer,
			num_indices,
		}
	}
}

pub struct MeshGenerator {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u16>,
}
impl Default for MeshGenerator {
	fn default() -> Self {
		Self {
			vertices: vec![],
			indices: vec![],
		}
	}
}
impl MeshGenerator {
	fn add_vertex(&mut self, vertex: Vertex) {
		self.vertices.push(vertex);
	}
	fn add_triangle(&mut self, a: u16, b: u16, c: u16) {
		self.indices.extend_from_slice(&[a, b, c]);
	}

	fn clear(&mut self) {
		self.vertices.clear();
		self.indices.clear();
	}
	pub fn build(&mut self, device: &wgpu::Device, label: Option<&str>) -> Mesh {
		let num_indices = self.indices.len() as u32;
		Mesh::new(
			device,
			label,
			&self.vertices[..],
			&self.indices[..],
			num_indices,
		)
	}
	// pub fn quad(&mut self) -> &mut Self {
	// 	self.clear();
	// 	self.vertices.extend_from_slice(&QUAD_VERTICES);
	// 	self.indices.extend_from_slice(&QUAD_INDICES);
	// 	self
	// }
	pub fn uv_sphere(
		&mut self,
		radius: f32,
		vertical_subdivisions: usize,
		horizontal_subdivisions: usize,
	) -> &mut Self {
		let start = Instant::now();
		self.clear();

		let vertex_count = (vertical_subdivisions + 1) * (horizontal_subdivisions + 1);
		let index_count = (vertical_subdivisions - 1) * horizontal_subdivisions * 6;

		self.vertices.reserve(vertex_count);
		self.indices.reserve(index_count);

		let mut sector_angle_cos = Vec::with_capacity(horizontal_subdivisions + 1);
		let mut sector_angle_sin = Vec::with_capacity(horizontal_subdivisions + 1);

		let stack_count = vertical_subdivisions;
		let sector_count = horizontal_subdivisions;

		let stack_count_f = stack_count as f32;
		let sector_count_f = sector_count as f32;

		let radius_inverse = 1.0 / radius;
		// bottom to top, 0 to PI
		let stack_step = PI / stack_count_f;
		// all the way around, 0 to 2 PI
		let sector_step = 2.0 * PI / sector_count_f;

		let mut stack_angle: f32 = PI * 0.5;
		for stack in 0..(stack_count + 1) {
			let y = radius * stack_angle.sin();
			let xz = radius * stack_angle.cos();

			let mut sector_angle: f32 = -PI;
			let v = stack as f32 / (stack_count_f + 1.0);
			for sector in 0..(sector_count + 1) {
				let (cos, sin) = if stack == 0 {
					let cos = sector_angle.cos();
					let sin = sector_angle.sin();
					sector_angle += sector_step;
					sector_angle_cos.push(cos);
					sector_angle_sin.push(sin);
					(cos, sin)
				} else {
					(sector_angle_cos[sector], sector_angle_sin[sector])
				};

				let z = xz * cos;
				let x = xz * sin;
				let position = [x, y, z];

				let nx = x * radius_inverse;
				let ny = y * radius_inverse;
				let nz = z * radius_inverse;
				let _normal = [nx, ny, nz];

				let mut color: [f32; 3] = [nx, ny, nz];
				color = color.map(|c| if c < 0.0 { -c } else { c });

				let u = sector as f32 / (sector_count_f);
				let uv = [u, v];

				let vertex = Vertex {
					position,
					color,
					uv,
				};
				self.add_vertex(vertex);
			}
			stack_angle -= stack_step;
		}

		for i in 0..stack_count {
			let mut k1 = (i * (sector_count + 1)) as u16;
			let mut k2 = k1 + sector_count as u16 + 1;

			for _ in 0..sector_count {
				// not bottom
				if i != 0 {
					self.add_triangle(k1, k2, k1 + 1)
				}

				// not top
				if i != (stack_count - 1) {
					self.add_triangle(k1 + 1, k2, k2 + 1)
				}

				k1 += 1;
				k2 += 1;
			}
		}

		let duration = start.elapsed();
		println!("{} us", duration.as_micros());

		self
	}
}

#[allow(unused)]
pub const TRIANGLE_VERTICES: &[Vertex] = &[
	Vertex {
		position: [0.0, 0.5, 0.0],
		color: [1.0, 0.0, 0.0],
		uv: [0.0, 0.0],
	},
	Vertex {
		position: [-0.5, -0.5, 0.0],
		color: [0.0, 1.0, 0.0],
		uv: [0.0, 0.0],
	},
	Vertex {
		position: [0.5, -0.5, 0.0],
		color: [0.0, 0.0, 1.0],
		uv: [0.0, 0.0],
	},
];
#[allow(unused)]
pub const TRIANGLE_INDICES: &[u16] = &[0, 1, 2];

#[allow(unused)]
pub const QUAD_VERTICES: &[Vertex] = &[
	Vertex {
		// lower left
		position: [-0.5, -0.5, 0.0],
		color: [0.0, 0.0, 1.0],
		uv: [0.0, 0.0],
	},
	Vertex {
		// lower right
		position: [0.5, -0.5, 0.0],
		color: [0.0, 1.0, 0.0],
		uv: [0.0, 1.0],
	},
	Vertex {
		// upper right
		position: [0.5, 0.5, 0.0],
		color: [1.0, 0.0, 0.0],
		uv: [1.0, 1.0],
	},
	Vertex {
		// upper left
		position: [-0.5, 0.5, 0.0],
		color: [0.0, 0.0, 1.0],
		uv: [1.0, 0.0],
	},
];
#[allow(unused)]
pub const QUAD_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

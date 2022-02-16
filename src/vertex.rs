// File: vertex.rs

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
	pub color: [f32; 3],
	// pub normal: [f32; 3],
	// pub tangent: [f32; 3],
	// pub bitangent: [f32; 3],
	// pub uv: [f32; 2],
}
impl Vertex {
	pub const ATTRIBS: [wgpu::VertexAttribute; 2] =
		wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

	pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
		use std::mem;

		wgpu::VertexBufferLayout {
			array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &Self::ATTRIBS,
		}
	}
}


use cgmath::Vector3;
use cgmath::VectorSpace;
use cgmath::InnerSpace;
use core::f32::consts::PI;

const TWO_PI: f32 = PI * 2.0;
const HALF_PI: f32 = PI / 2.0;

fn map_longitude(val: f32, h_1: f32) -> f32 {
	-PI + val * TWO_PI / h_1
}
fn map_latitude(val: f32, h_1: f32) -> f32 {
	-HALF_PI + val * PI / h_1
}

pub struct MeshGenerator {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u16>,
}
impl Default for MeshGenerator {
	fn default() -> Self {
		Self {
			vertices: vec!(),
			indices: vec!(),
		}
	}
}
impl MeshGenerator {
	pub fn add_vertex(&mut self, vertex: Vertex) {
		self.vertices.push(vertex);
	}
	pub fn add_triangle(&mut self, a: u16, b: u16, c: u16) {
		self.indices.extend_from_slice(&[a, b, c]);
	}

	pub fn clear(&mut self) {
		self.vertices.clear();
		self.indices.clear();
	}
	pub fn uv_sphere(&mut self, radius: f32, vertical_subdivisions: usize, horizontal_subdivisions: usize) {
		let vertex_count =
			(vertical_subdivisions + 1) * (horizontal_subdivisions + 1);
		let index_count = (vertical_subdivisions - 1) * horizontal_subdivisions * 6;

		self.vertices.reserve(vertex_count);
		self.indices.reserve(index_count);

		let mut sector_angle_cos = Vec::with_capacity(horizontal_subdivisions + 1);
		let mut sector_angle_sin = Vec::with_capacity(horizontal_subdivisions + 1);

		let stack_count = vertical_subdivisions as u32;
		let sector_count = horizontal_subdivisions as u32;

		let stack_count_f = stack_count as f32;
		let sector_count_f = sector_count as f32;

		let radius_inverse = 1.0 / radius;
		let sector_step = 2.0 * PI / sector_count_f;
		let stack_step = PI / stack_count_f;


		let mut min = 20.0;
		let mut max = -20.0;


		let mut stack_angle = PI * 1.5;
		for stack in 0..(stack_count + 1) {
			let xz = radius * stack_angle.cos();
			let y = radius * stack_angle.sin();

			let mut sector_angle: f32 = 0.0;
			let texture_coord_t = stack as f32 / stack_count_f;
			for sector in 0..(sector_count + 1) {
				let (cos, sin) = if stack == 0 {
					let cos = sector_angle.cos();
					let sin = sector_angle.sin();
					sector_angle += sector_step;
					sector_angle_cos.push(cos);
					sector_angle_sin.push(sin);
					(cos, sin)
				} else {
					(
						sector_angle_cos[sector as usize],
						sector_angle_sin[sector as usize],
					)
				};

				let x = xz * cos;
				let z = xz * sin;
				let position = Vector3::new(x, y, z);

				let nx = x * radius_inverse;
				let ny = y * radius_inverse;
				let nz = z * radius_inverse;
				let normal = Vector3::new(nx, ny, nz);

				// let texture_coord_s = sector as f32 / sector_count_f;
				// let texcoord_0 = Some(Vector2::new(texture_coord_s, texture_coord_t));

				let mut color: [f32; 3] = normal.into();
				color = color.map(|v| if v < 0.0 { -v } else { v });
				if color[0] > max {
					max = color[0];
				}
				if color[0] < min {
					min = color[0];
				}
				let vertex = Vertex {
					position: position.into(),
					color,
				};
				self.add_vertex(vertex);
			}
			stack_angle -= stack_step;
		}

		for i in 0..stack_count {
			let mut k1 = (i * (sector_count + 1)) as u16;
			let mut k2 = (k1 + sector_count as u16 + 1 ) as u16;

			for _ in 0..sector_count {
				if i != 0 {
					self.add_triangle(k1, k2, k1 + 1)
				}

				if i != (stack_count - 1) {
					self.add_triangle(k1 + 1, k2, k2 + 1)
				}

				k1 += 1;
				k2 += 1;
			}
		}
	}
}
const ORIGINS: &[cgmath::Vector3<f32>] = &[
	cgmath::Vector3::new(-1.0,-1.0,-1.0),
	cgmath::Vector3::new( 1.0,-1.0,-1.0),
	cgmath::Vector3::new( 1.0,-1.0, 1.0),
	cgmath::Vector3::new(-1.0,-1.0, 1.0),
	cgmath::Vector3::new(-1.0, 1.0,-1.0),
	cgmath::Vector3::new(-1.0,-1.0, 1.0),
];
const RIGHTS: &[cgmath::Vector3<f32>] = &[
	cgmath::Vector3::new( 1.0, 0.0, 0.0),
	cgmath::Vector3::new( 0.0, 0.0, 1.0),
	cgmath::Vector3::new(-1.0, 0.0, 0.0),
	cgmath::Vector3::new( 0.0, 0.0,-1.0),
	cgmath::Vector3::new( 1.0, 0.0, 0.0),
	cgmath::Vector3::new( 1.0, 0.0, 0.0),
];
const UPS: &[cgmath::Vector3<f32>] = &[
	cgmath::Vector3::new(0.0, 1.0, 0.0),
	cgmath::Vector3::new(0.0, 1.0, 0.0),
	cgmath::Vector3::new(0.0, 1.0, 0.0),
	cgmath::Vector3::new(0.0, 1.0, 0.0),
	cgmath::Vector3::new(0.0, 0.0, 1.0),
	cgmath::Vector3::new(0.0, 0.0,-1.0),
];
// File: mesh.rs

use crate::vertex::*;

use wgpu::util::DeviceExt;

pub struct Mesh {
	pub vertex_buffer_label: Option<String>,
	pub vertex_buffer: wgpu::Buffer,

	pub index_buffer_label: Option<String>,
	pub index_buffer: wgpu::Buffer,
	pub num_indices: u32,
}
impl Mesh {
	pub fn new(device: &wgpu::Device, label: Option<&str>, vertices: &[Vertex], indices: &[u16], num_indices: u32) -> Self {
		let (vertex_buffer_label, index_buffer_label) = match label {
			Some(l) => {
				(Some(format!("{} vertex buffer label", l)), Some(format!("{} index buffer label", l)))
			}
			None => (None, None)
		};

		let vertex_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: vertex_buffer_label.as_ref().map(|v| v.as_str()),
				contents: bytemuck::cast_slice(vertices),
				usage: wgpu::BufferUsages::VERTEX,
			}
		);

		let index_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: index_buffer_label.as_ref().map(|v| v.as_str()),
				contents: bytemuck::cast_slice(indices),
				usage: wgpu::BufferUsages::INDEX,
			}
		);

		Self {
			vertex_buffer_label,
			vertex_buffer,
			index_buffer_label,
			index_buffer,
			num_indices,
		}
	}
}
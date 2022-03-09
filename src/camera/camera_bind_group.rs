// File: camera/camera_bind_group.rs

use wgpu::util::DeviceExt;

use super::camera_uniform::CameraUniform;
use super::Camera;
use bevy::prelude::Component;

#[derive(Component)]
pub struct CameraBindGroup {
	pub uniform: CameraUniform,
	pub buffer: wgpu::Buffer,
	pub layout: wgpu::BindGroupLayout,
	pub bind_group: wgpu::BindGroup,
}
impl CameraBindGroup {
	pub fn new(device: &wgpu::Device, label: Option<&str>, camera: &Camera) -> Self {
		let mut uniform = CameraUniform::new();
		uniform.set_view_proj(camera);

		let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("camera buffer"),
			contents: bytemuck::cast_slice(&[uniform]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let layout = Self::create_layout(device);

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: buffer.as_entire_binding(),
			}],
			label,
		});

		CameraBindGroup {
			uniform,
			buffer,
			layout,
			bind_group,
		}
	}
	pub fn create_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
		device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			entries: &[wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStages::VERTEX,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: None,
				},
				count: None,
			}],
			label: Some("camera_layout"),
		})
	}
}

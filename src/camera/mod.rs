// File: camera/mod.rs

mod camera_bind_group;
mod camera_controller;
mod camera_uniform;
mod projection;

pub use camera_bind_group::CameraBindGroup;
pub use camera_controller::CameraController;
pub use camera_uniform::CameraUniform;
pub use projection::Projection;

use cgmath::SquareMatrix;

use bevy::prelude::Component;

#[derive(Component)]
pub struct PrimaryCamera;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
	1.0, 0.0, 0.0, 0.0,
	0.0, 1.0, 0.0, 0.0,
	0.0, 0.0, 0.5, 0.0,
	0.0, 0.0, 0.5, 1.0,
);

#[derive(Copy, Clone, Debug, Component)]
pub struct Camera {
	pub eye: cgmath::Point3<f32>,
	pub target: cgmath::Point3<f32>,
	pub up: cgmath::Vector3<f32>,
	projection: Projection,
	view_matrix: cgmath::Matrix4<f32>,
	view_projection_matrix: cgmath::Matrix4<f32>,
}
impl Camera {
	pub fn new(
		eye: cgmath::Point3<f32>,
		target: cgmath::Point3<f32>,
		projection: &Projection,
	) -> Self {
		let up = cgmath::Vector3::new(0.0, 1.0, 0.0); // eye.to_vec().cross(cgmath::Vector3::new(0.0, 0.0, 0.0));
		let mut camera = Self {
			eye,
			target,
			up,
			projection: *projection,
			view_matrix: cgmath::Matrix4::identity(),
			view_projection_matrix: cgmath::Matrix4::identity(),
		};
		camera.update_view_matrix();
		camera
	}
	pub fn update_projection_matrix(&mut self) {
		self.projection.update_projection_matrix();
		self.update_view_projection_matrix();
	}
	pub fn update_view_matrix(&mut self) {
		self.view_matrix = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
		self.update_view_projection_matrix();
	}
	fn update_view_projection_matrix(&mut self) {
		self.view_projection_matrix =
			OPENGL_TO_WGPU_MATRIX * self.projection.matrix * self.view_matrix;
	}

	pub fn set_eye(&mut self, eye: cgmath::Point3<f32>) {
		self.eye = eye;
		self.update_view_matrix();
	}
	pub fn set_target(&mut self, target: cgmath::Point3<f32>) {
		self.target = target;
		self.update_view_matrix();
	}
	pub fn set_up(&mut self, up: cgmath::Vector3<f32>) {
		self.up = up;
		self.update_view_matrix();
	}

	pub fn set_aspect(&mut self, width: u32, height: u32) {
		self.projection.aspect = Projection::aspect(width, height);
		self.update_projection_matrix();
	}
}

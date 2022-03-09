// File: camera/camera_uniform.rs

use super::Camera;
use cgmath::SquareMatrix;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
	view_projection_matrix: [[f32; 4]; 4],
}
impl CameraUniform {
	pub fn new() -> Self {
		Self {
			view_projection_matrix: cgmath::Matrix4::identity().into(),
		}
	}
	pub fn set_view_proj(&mut self, camera: &Camera) {
		self.view_projection_matrix = camera.view_projection_matrix.into();
	}
}

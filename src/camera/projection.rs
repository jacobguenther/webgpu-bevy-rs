// File: camera/projection.rs

#[derive(Copy, Clone, Debug)]
pub struct Projection {
	pub aspect: f32,
	pub fovy: f32,
	pub znear: f32,
	pub zfar: f32,
	pub matrix: cgmath::Matrix4<f32>,
}
impl Projection {
	pub fn new(width: u32, height: u32, fovy: f32, znear: f32, zfar: f32) -> Self {
		let aspect = Self::aspect(width, height);
		Projection {
			aspect,
			fovy,
			znear,
			zfar,
			matrix: cgmath::perspective(cgmath::Deg(fovy), aspect, znear, zfar),
		}
	}
	pub fn update_projection_matrix(&mut self) {
		self.matrix =
			cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
	}
	pub fn aspect(width: u32, height: u32) -> f32 {
		width as f32 / height as f32
	}
}

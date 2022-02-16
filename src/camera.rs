// File: projection.rs

use cgmath::SquareMatrix;
use cgmath::Zero;

use winit::event::WindowEvent;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
	1.0, 0.0, 0.0, 0.0,
	0.0, 1.0, 0.0, 0.0,
	0.0, 0.0, 0.5, 0.0,
	0.0, 0.0, 0.5, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
	view_projection_matrix: [[f32; 4]; 4],
}
impl CameraUniform {
	pub fn new() -> Self {
		use cgmath::SquareMatrix;
		Self {
			view_projection_matrix: cgmath::Matrix4::identity().into(),
		}
	}
	pub fn load_view_proj(&mut self, camera: &Camera) {
		self.view_projection_matrix = camera.view_projection_matrix.into();
	}
}

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
	fn update_projection_matrix(&mut self) {
		self.matrix =
			cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
	}
	fn aspect(width: u32, height: u32) -> f32 {
		width as f32 / height as f32
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Camera {
	eye: cgmath::Point3<f32>,
	target: cgmath::Point3<f32>,
	up: cgmath::Vector3<f32>,
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
		let mut camera = Self {
			eye,
			target,
			up: cgmath::Vector3::unit_y(),
			projection: *projection,
			view_matrix: cgmath::Matrix4::identity(),
			view_projection_matrix: cgmath::Matrix4::identity(),
		};
		camera.update_view_matrix();
		camera
	}
	fn update_projection_matrix(&mut self) {
		self.projection.update_projection_matrix();
		self.update_view_projection_matrix();
	}
	fn update_view_matrix(&mut self) {
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

pub struct CameraController {
	speed: f32,
	forward_pressed: bool,
	backward_pressed: bool,
	right_pressed: bool,
	left_pressed: bool,
}
impl CameraController {
	pub fn new(speed: f32) -> Self {
		Self {
			speed,
			forward_pressed: false,
			backward_pressed: false,
			right_pressed: false,
			left_pressed: false,
		}
	}
	pub fn process_events(&mut self, event: &WindowEvent) -> bool {
		todo!()
	}
	pub fn update_camera(&self, camera: &mut Camera) {
		todo!()
	}
}

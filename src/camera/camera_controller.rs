// File: camera/camera_controller.rs

use cgmath::InnerSpace;

use super::Camera;
use bevy::prelude::Component;
use bevy::{input::keyboard::KeyCode, prelude::*};

#[derive(Component)]
pub struct CameraController {
	pub speed: f32,
	pub forward_pressed: bool,
	pub backward_pressed: bool,
	pub right_pressed: bool,
	pub left_pressed: bool,
	pub up_pressed: bool,
	pub down_pressed: bool,
}
impl Default for CameraController {
	fn default() -> Self {
		CameraController::new(0.1)
	}
}
impl CameraController {
	pub fn new(speed: f32) -> Self {
		Self {
			speed,
			forward_pressed: false,
			backward_pressed: false,
			right_pressed: false,
			left_pressed: false,
			up_pressed: false,
			down_pressed: false,
		}
	}
	pub fn process_events(&mut self, keys: Res<Input<KeyCode>>) -> bool {
		self.forward_pressed = keys.pressed(KeyCode::Q);
		self.backward_pressed = keys.pressed(KeyCode::E);
		self.right_pressed = keys.pressed(KeyCode::D);
		self.left_pressed = keys.pressed(KeyCode::A);
		self.up_pressed = keys.pressed(KeyCode::W);
		self.down_pressed = keys.pressed(KeyCode::S);
		self.forward_pressed
			| self.backward_pressed
			| self.right_pressed
			| self.left_pressed
			| self.up_pressed
			| self.down_pressed
	}
	pub fn update_camera(&self, camera: &mut Camera) {
		let forward = camera.target - camera.eye;
		let forward_norm = forward.normalize();
		let forward_mag = forward.magnitude();

		if self.forward_pressed && forward_mag > self.speed {
			camera.eye += forward_norm * self.speed;
		}
		if self.backward_pressed {
			camera.eye -= forward_norm * self.speed;
		}

		let forward = camera.target - camera.eye;
		let forward_mag = forward.magnitude();

		let right = forward_norm.cross(camera.up);

		if self.right_pressed {
			camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
		}
		if self.left_pressed {
			camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
		}

		if self.up_pressed {
			camera.eye =
				camera.target - (forward - camera.up * self.speed).normalize() * forward_mag;
		}
		if self.down_pressed {
			camera.eye =
				camera.target - (forward + camera.up * self.speed).normalize() * forward_mag;
		}
	}
}

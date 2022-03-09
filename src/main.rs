// File: main.rs

mod camera;
mod mesh;
mod render_state;
mod texture;
mod vertex;

use camera::{Camera, CameraBindGroup, CameraController, PrimaryCamera, Projection};
use mesh::{Mesh, MeshGenerator, ShouldDraw};
use render_state::*;

use bevy::{
	app::App, ecs::system::Commands, input::keyboard::KeyCode, prelude::*, window::WindowResized,
	winit::WinitWindows,
};

fn init_renderer(windows: Res<WinitWindows>, mut commands: Commands) {
	let (_id, window) = windows.as_ref().windows.iter().next().unwrap();
	let renderer = pollster::block_on(RenderState::new(&window));

	{
		let camera = {
			let projection = {
				let fovy = 45.0;
				let znear = 0.1;
				let zfar = 1000.0;
				Projection::new(renderer.size.width, renderer.size.height, fovy, znear, zfar)
			};
			let eye = cgmath::Point3::new(0.0, 0.0, 2.0);
			let target = cgmath::Point3::new(0.0, 0.0, 0.0);
			Camera::new(eye, target, &projection)
		};
		let camera_bind_group =
			CameraBindGroup::new(&renderer.device, Some("camera_bind_group"), &camera);
		let camera_controller = CameraController::default();

		commands
			.spawn()
			.insert(PrimaryCamera {})
			.insert(camera)
			.insert(camera_bind_group)
			.insert(camera_controller);
	}

	{
		let radius = 0.25;
		let divs = 64;
		let sphere = MeshGenerator::default()
			.uv_sphere(radius, divs, divs)
			.build(&renderer.device, Some("sphere_mesh"));
		commands.spawn().insert(sphere).insert(ShouldDraw {});
	}

	commands.insert_resource(renderer);
}
fn render(
	renderer: Res<RenderState>,
	camera_query: Query<&CameraBindGroup, With<PrimaryCamera>>,
	mesh_query: Query<&Mesh, With<ShouldDraw>>,
) {
	let renderer = renderer.as_ref();
	let camera_bind_group = camera_query.iter().next().unwrap();
	let _result = renderer.render(camera_bind_group, mesh_query.iter());
}
fn window_resize(
	mut reader: EventReader<WindowResized>,
	mut renderer: ResMut<RenderState>,
	mut camera_query: Query<(&mut Camera, &mut CameraBindGroup), With<PrimaryCamera>>,
) {
	if let Some(event) = reader.iter().next() {
		let (width, height) = (event.width as u32, event.height as u32);
		renderer.as_mut().resize(width, height);

		let (mut camera, mut camera_bind_group) = camera_query.iter_mut().next().unwrap();
		camera.set_aspect(width, height);
		camera_bind_group.uniform.set_view_proj(&camera);
		renderer.queue.write_buffer(
			&camera_bind_group.buffer,
			0,
			bytemuck::cast_slice(&[camera_bind_group.uniform]),
		);
	}
}
pub fn camera_controls(
	keys: Res<Input<KeyCode>>,
	renderer: Res<RenderState>,
	mut camera_query: Query<
		(&mut Camera, &mut CameraController, &mut CameraBindGroup),
		With<PrimaryCamera>,
	>,
) {
	let (mut camera, mut camera_controller, mut camera_bind_group) =
		camera_query.iter_mut().next().unwrap();

	let changed = camera_controller.process_events(keys);
	if changed {
		camera_controller.update_camera(&mut camera);
		camera.update_view_matrix();

		camera_bind_group.uniform.set_view_proj(&camera);
		renderer.queue.write_buffer(
			&camera_bind_group.buffer,
			0,
			bytemuck::cast_slice(&[camera_bind_group.uniform]),
		);
	}
}

fn main() {
	env_logger::init();

	App::new()
		.add_plugin(bevy::window::WindowPlugin::default())
		.add_plugin(bevy::input::InputPlugin::default())
		.add_plugin(bevy::winit::WinitPlugin::default())
		.add_startup_system(init_renderer.system())
		.add_system(render.system())
		.add_system(window_resize.system())
		.add_system(camera_controls.system())
		.run();
}

// File: main.rs

mod primitives;
mod vertex;
mod mesh;
mod render_state;
mod camera;
mod texture;

use render_state::*;

use winit::{
	event::*,
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

use bevy::{
	app::App,
	app::Events,
	ecs::system::Commands,
	ecs::system::EntityCommands,
	prelude::*,
	winit::WinitWindows,
	window::WindowResized,
};

fn init_renderer(mut windows: ResMut<WinitWindows>, mut commands: Commands) {
	let (id, window) = windows.as_ref().windows.iter().next().unwrap();
	let renderer = pollster::block_on(RenderState::new(&window));
	commands.insert_resource(renderer);
}
fn render(mut renderer: ResMut<RenderState>) {
	let mut renderer = renderer.as_mut();
	renderer.render();
}
fn resize(mut reader: EventReader<WindowResized>, mut renderer: ResMut<RenderState>) {
	if let Some(event) = reader.iter().next() {
		renderer.as_mut().resize(event.width as u32, event.height as u32);
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
		.add_system(resize.system())
		.run();
}


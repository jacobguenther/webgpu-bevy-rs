// Vertex shader

struct CameraUniform {
	view_projection_matrix: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> camera: CameraUniform;

struct VertexInput {
	[[location(0)]] position: vec3<f32>;
	[[location(1)]] color: vec3<f32>;
	[[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
	[[builtin(position)]] clip_position: vec4<f32>;
	[[location(0)]] color: vec3<f32>;
	[[location(1)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
	model: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;
	out.color = model.color;
	out.uv = model.uv;
	out.clip_position = camera.view_projection_matrix * vec4<f32>(model.position, 1.0);
	return out;
}


// Fragment shader
[[group(1), binding(0)]]
var diffuse_texture: texture_2d<f32>;
[[group(1), binding(1)]]
var diffuse_sampler: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var diffuse = textureSample(diffuse_texture, diffuse_sampler, in.uv);
	return diffuse;
	// return vec4<f32>(in.color, 1.0);
}
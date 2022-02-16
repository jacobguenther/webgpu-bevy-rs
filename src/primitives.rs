// File: primitives.rs

use crate::vertex::*;

#[allow(unused)]
pub const TRIANGLE_VERTICES: &[Vertex] = &[
	Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
	Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
	Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];
#[allow(unused)]
pub const TRIANGLE_INDICES: &[u16] = &[
	0, 1, 2
];

#[allow(unused)]
pub const QUAD_VERTICES: &[Vertex] = &[
	Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
	Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
	Vertex { position: [ 0.5,  0.5, 0.0], color: [1.0, 0.0, 0.0] },
	Vertex { position: [-0.5,  0.5, 0.0], color: [0.0, 0.0, 1.0] },
];
#[allow(unused)]
pub const QUAD_INDICES: &[u16] = &[
	0, 1, 2,
	0, 2, 3,
];
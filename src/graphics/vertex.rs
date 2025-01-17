#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            position,
            tex_coords,
            color,
        }
    }
}

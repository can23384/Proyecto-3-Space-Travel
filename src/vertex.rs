use raylib::math::{Vector2, Vector3};

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub tex_coords: Vector2,
    pub color: Vector3,
    pub transformed_position: Vector3,
    pub transformed_normal: Vector3,
}

impl Vertex {
    pub fn new(position: Vector3, normal: Vector3, tex_coords: Vector2) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
            color: Vector3::new(1.0, 1.0, 1.0),
            transformed_position: position,
            transformed_normal: normal,
        }
    }
}

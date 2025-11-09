use crate::vertex::Vertex;
use raylib::math::{Vector2, Vector3};
use tobj;

pub struct Obj {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Obj {
    pub fn load(path: &str) -> Result<Self, tobj::LoadError> {
        let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for model in models {
            let mesh = &model.mesh;
            for i in 0..(mesh.positions.len() / 3) {
                let position = Vector3::new(mesh.positions[i * 3], -mesh.positions[i * 3 + 1], mesh.positions[i * 3 + 2]);
                let normal = if !mesh.normals.is_empty() {
                    Vector3::new(mesh.normals[i * 3], mesh.normals[i * 3 + 1], mesh.normals[i * 3 + 2])
                } else {
                    Vector3::zero()
                };
                let tex = if !mesh.texcoords.is_empty() {
                    Vector2::new(mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1])
                } else {
                    Vector2::zero()
                };
                vertices.push(Vertex::new(position, normal, tex));
            }
            indices.extend_from_slice(&mesh.indices);
        }

        Ok(Obj { vertices, indices })
    }
}

use cgmath::{Vector3};
use mesh;
struct Object{
    position : Vector3<f32>,
    rotation : Vector3<f32>,
    scale : Vector3<f32>,
    color : Option<Vector3<f32>>,
    mesh : mesh::Mesh,
}
impl Object{
    pub fn new(mesh : mesh::Mesh)-> Object{
        Object{
            position : Vector3::new(0.0,0.0,0.0),
            rotation : Vector3::new(0.0,0.0,0.0),
            scale    : Vector3::new(1.0,1.0,1.0),
            color    : None,
            mesh : mesh,
        }
    }
}
use cgmath::{Vector3,Vector2};

struct Vertex{
    position  : [f32;3],
    normal    : [f32;3],
    //texture_c : Vector3<f32>,
    //color     : Vector3<f32>,
}

pub struct Mesh{
    vertices : Vec<Vertex>,
    indices  : Vec<u32>,
}
impl Mesh{
    pub fn Quad(width : f32,height : f32)->Mesh{
        let vertices = vec![Vertex{position:[0.5,-0.5,0.0],normal:[0.0,0.0,0.0]},Vertex{position:[-0.5,-0.5,0.0],normal:[0.5,-0.5,0.0]},Vertex{position:[-0.5,0.5,0.0],normal:[0.5,-0.5,0.0]},Vertex{position:[0.5,0.5,0.0],normal:[0.5,-0.5,0.0]}];
      Mesh{vertices,indices : vec![0, 1, 2, 2, 3, 0]}
    }

}
use renderer;
use cgmath::{Vector2};

mod Shape{
    use super::renderer::Vertex;
    pub fn create_rectangle(height: f32,width: f32)->Vec<Vertex>{
        return vec![Vertex{pos: [0.5, -0.5],uv : [1.0, 1.0]},Vertex{pos: [-0.5, -0.5],uv : [1.0, 1.0]},Vertex { pos: [-0.5, 0.5],uv : [1.0, 1.0]},Vertex { pos: [-0.5, 0.5],uv : [1.0, 1.0]},Vertex { pos: [0.5, 0.5],uv : [1.0, 1.0]},Vertex { pos: [0.5, -0.5],uv : [1.0, 1.0]}];
    }
    pub fn create_triangle(height:f32,width:f32)->Vec<super::renderer::Vertex>{
        return vec![];
    }
    pub fn create_cirle(radius:f32)->Vec<super::renderer::Vertex>{
        return vec![];
    }
}

pub struct Object{
    vertexdata : Vec<renderer::Vertex>,
    gpudata:Option<renderer::GpuData>,
    pub update : bool,
    position : Vector2<f32>,
    roation : u8,
    color : [f32;3],
}
impl Object{
    pub fn new(vertexdata : Vec<renderer::Vertex>)->Self{
        Object{vertexdata:vertexdata,gpudata:None,update : true,position:Vector2::new(0.0,0.0), roation:0,color:[1.0,1.0,1.0]}
    }
}


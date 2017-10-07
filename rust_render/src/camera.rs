use cgmath::{ortho,Vector3,Matrix4};

pub struct Camera{
    pub projection : Matrix4<f32>,
    pub position : Vector3<f32>,
}

impl Camera{
    pub fn new(width:u32,height:u32)->Self{
        Camera{
            projection : ortho(0.0,width as f32,0.0,height as f32,-1.0,100.0),
            position : Vector3::new(0.0,0.0,-1.0),
        }
    }
}
use renderer;
use renderer::RenderType;
use cgmath::{Vector2,Vector3};
//use std::f32::consts::PI;
use std::f32;

pub mod Shape{
    use super::renderer::RenderType;
    use super::renderer::Vertex;

    pub fn create_rectangle(width: f32,height: f32)->(Vec<Vertex>,RenderType){
        return (vec![Vertex{pos: [width*1.0, height*-1.0],uv : [1.0, 1.0]},Vertex{pos: [width*-1.0, height*-1.0],uv : [1.0, 1.0]},Vertex { pos: [width*-1.0, height*1.0],uv : [1.0, 1.0]},Vertex { pos: [width*-1.0, height*1.0],uv : [1.0, 1.0]},Vertex { pos: [width*1.0, height*1.0],uv : [1.0, 1.0]},Vertex { pos: [width*1.0, height*-1.0],uv : [1.0, 1.0]}],RenderType::shape);
    }
    pub fn create_triangle(height:f32,width:f32)->(Vec<Vertex>,RenderType){

        return (vec![Vertex{pos: [width*0.0,height*1.0],uv:[1.0,1.0]},Vertex{pos: [width*-1.0,height*-1.0],uv:[1.0,1.0]},Vertex{pos: [width*1.0,height*-1.0],uv:[1.0,1.0]}],RenderType::shape);
    }
    pub fn create_cirle(radius:f32)->(Vec<Vertex>,RenderType){
        let mut vertex = Vec::new();
        let twicePi = 2.0 * 3.1415926535;
        for x in 0..20{
            vertex.push(Vertex{pos:[0.0 + (radius*f32::cos(x as f32 *  twicePi / 20.0)),0.0 + (radius * f32::sin(x as f32 * twicePi / 20.0))],uv:[1.0,1.0]});
        }
        return (vertex,RenderType::shape);
    }
}

pub struct Object{
    pub vertexdata : Vec<renderer::Vertex>,
    pub gpudata: renderer::GpuData,
    pub update : bool,
    pub position : Vector3<f32>,
    shape_type: RenderType,
    scale : Vector2<f32>,
    roation : u8,
    color : [f32;4],
}
impl Object{
    pub fn new((vertexdata,render_type):( Vec<renderer::Vertex>,RenderType))->Self{
        Object{vertexdata:vertexdata,gpudata: renderer::GpuData{slice:None,vertices:None,constants:None},update:true,position:Vector3::new(0.0,0.0,-1.0), roation:0,color:[1.0,1.0,1.0,1.0],scale:Vector2::new(1.0,1.0),shape_type:render_type}
    }
    pub fn set_position(&mut self,x : f32,y: f32) {
        self.position.x = x;
        self.position.y = y;
    }
    pub fn get_position(&self) ->(f32,f32) {
        return (self.position.x,self.position.y);
    }
    pub fn set_color(&mut self,color: [f32;3]){

        self.color = [color[0],color[1],color[2],self.color[3]];
    }
    pub fn get_color(&self)->[f32;4]{
        return self.color;
    }
    pub fn set_opacity(&mut self,opacity : f32) {
        let mut opa = opacity;
        if opa > 1.0 {
            opa = 1.0;
        }else if opa < 0.0 {
            opa  = 0.0;
        }
        self.color[3] = opa;
    }
}


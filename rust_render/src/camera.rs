use cgmath::{ortho,Vector3,Matrix4,Vector2};

pub struct Camera{
    pub projection : Matrix4<f32>,
    pub position : Vector3<f32>,
    scale : f32,
    size : Vector2<u32>,
}

impl Camera{
    pub fn new(width:u32,height:u32)->Self{
        Camera{
            projection : ortho(width as f32,0.0,0.0,height as f32,0.0,100.0),
            position : Vector3::new(0.0,0.0,-1.0),
            scale : 1.0,
            size : Vector2::new(width,height),
        }
    }
    pub fn set_scale(&mut self,scale:f32){
        let mut scale = scale;
        if scale > 1.0{
            scale = 1.0;
        }else if scale < 0.05{
            scale = 0.05;
        }
        self.scale = scale;

        let width = self.size.x as f32 * scale;
        let height = self.size.y as f32 * scale;
        self.projection = ortho(0.0, width,0.0,height,0.0,100.0);
        self.position.x += (self.size.x as f32 - width);
        self.position.y += (self.size.y as f32 - height);
        println!("{:?}",self.position);
    }
    pub fn get_scale(&self)->f32{
        return self.scale;
    }
    pub fn set_position(&mut self,x:f32,y:f32){
        self.position.x = x;
        self.position.y = y;
    }
    pub fn get_position(&self)->(f32,f32){
        (self.position.x,self.position.y)
    }
    pub fn move_camera(&mut self,x:f32,y:f32){
        self.position.x += x;
        self.position.y += y;
    }
    pub fn resize(&mut self,width:u32,height:u32){
        self.projection = ortho(0.0,width as f32,0.0,height as f32,0.0,100.0);
        self.scale = 1.0;
        self.size = Vector2::new(width,height);
    }
}
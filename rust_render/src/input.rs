use glutin;
use std::collections::HashSet;
use cgmath::Vector2;

const PIXELS_PER_LINE: f32 = 38.0;

// to handle simple inputs
pub struct Input{
    pressed  : HashSet<glutin::VirtualKeyCode>,
    released : HashSet<glutin::VirtualKeyCode>,
    mouse_pressed : HashSet<glutin::MouseButton>,
    mouse_released : HashSet<glutin::MouseButton>,
    mouse_delta : f32,
    mouse_position : Vector2<f32>,
}

impl Input{
    pub fn new() -> Self{
        Input {
            pressed: HashSet::new(),
            released: HashSet::new(),
            mouse_pressed: HashSet::new(),
            mouse_released: HashSet::new(),
            mouse_delta : 0.0,
            mouse_position : Vector2::new(0.0,0.0),
        }
    }
    pub fn clear(&mut self){
        self.released.clear();
        self.pressed.clear();
        self.mouse_released.clear();
        self.mouse_pressed.clear();
        self.mouse_delta = 0.0;
    }
    pub fn add_key(&mut self,state : glutin::ElementState,key : glutin::VirtualKeyCode){
        if state == glutin::ElementState::Pressed {
            self.pressed.insert(key);
        }else { self.released.insert(key); }
    }
    pub fn add_button(&mut self,state : glutin::ElementState,button : glutin::MouseButton)
    {
        if state == glutin::ElementState::Pressed {
            self.mouse_pressed.insert(button);
        }else { self.mouse_released.insert(button); }
    }
    pub fn add_mouse_wheel(&mut self ,delta : glutin::MouseScrollDelta){
        match delta{
            glutin::MouseScrollDelta::LineDelta(_,y)=> self.mouse_delta = y * PIXELS_PER_LINE,
            glutin::MouseScrollDelta::PixelDelta(_,y) => (),
        }

    }
    pub fn add_mouse_position(&mut self,x : f32,y : f32) {
        self.mouse_position.x = x;
        self.mouse_position.y = y;
    }

    pub fn is_key_pressed(&mut self,key : glutin::VirtualKeyCode)->bool{
        return self.pressed.contains(&key);
    }
    pub fn is_key_released(&mut self,key : glutin::VirtualKeyCode)->bool{
        return self.released.contains(&key);
    }
    pub fn is_button_pressed(&mut self,button : glutin::MouseButton)->bool{
        return  self.mouse_pressed.contains(&button);
    }
    pub fn is_button_released(&mut self,button : glutin::MouseButton)->bool{
        return  self.mouse_released.contains(&button);
    }
    pub fn get_mouse_scroll(&self)->f32{
        return self.mouse_delta;
    }
    pub fn get_mouse_position(&self)->Vector2<f32>{
        return  self.mouse_position;
    }


}
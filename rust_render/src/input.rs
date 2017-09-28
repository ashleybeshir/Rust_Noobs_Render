use glutin;
use std::collections::HashSet;

// to handle simple inputs
pub struct Input{
    pressed  : HashSet<glutin::VirtualKeyCode>,
    released : HashSet<glutin::VirtualKeyCode>,
}

impl Input{
    pub fn new() -> Self{
        Input {
            pressed: HashSet::new(),
            released: HashSet::new(),
        }
    }
    pub fn clear(&mut self){
        self.released.clear();
        self.pressed.clear();
    }
    pub fn add_key(&mut self,state : glutin::ElementState,key : glutin::VirtualKeyCode){
        if state == glutin::ElementState::Pressed {
            self.pressed.insert(key);
        }else { self.released.insert(key); }
    }
    pub fn is_pressed(&mut self,key : glutin::VirtualKeyCode)->bool{
        return self.pressed.contains(&key);

    }
    pub fn is_released(&mut self,key : glutin::VirtualKeyCode)->bool{
        return self.released.contains(&key);

    }


}
use glutin;
use std::collections::HashSet;

// to handle simple inputs
pub struct Input{
    pressed  : HashSet<glutin::VirtualKeyCode>,
    released : HashSet<glutin::VirtualKeyCode>,
    temp : HashSet<glutin::VirtualKeyCode>,
}

impl Input{
    pub fn new() -> Self{
        Input {
            pressed: HashSet::new(),
            released: HashSet::new(),
            temp : HashSet::new(),
        }
    }
    pub fn add_key(&mut self,key : glutin::VirtualKeyCode){
        self.temp.insert(key);
    }

}
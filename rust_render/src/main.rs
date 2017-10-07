#[macro_use] extern crate gfx;

extern crate cgmath;
extern crate image;
#[cfg(feature = "opengl")]
extern crate gfx_device_gl;
#[cfg(feature = "opengl")]
extern crate gfx_window_glutin;
#[cfg(feature = "opengl")]
extern crate glutin;

mod renderer;
mod window;
mod input;
mod object;
mod camera;
use camera::Camera;

const width:u32 = 800;
const height:u32 = 600;

fn main() {
    let mut window = window::Window::new("Test Screen",width,height,false,false);
    let mut camera : Camera = Camera::new(width,height);
    let mut quad = object::Object::new(object::Shape::create_rectangle(8000.0,8000.0));
    while window.running{
        window.get_events();
        if window.input.is_key_pressed(glutin::VirtualKeyCode::O){
           quad.set_opacity(0.2);
        }
        if window.input.is_key_released(glutin::VirtualKeyCode::A){
            quad.set_color([0.5,0.0,0.0]);
        }
        window.set_camera(&camera);
        window.render(&mut quad);
       // window.render();
        window.display();
    }
    println!("Hello, world!");
}

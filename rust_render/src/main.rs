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


fn main() {
    let mut window = window::Window::new("Test Screen",800,600,false,false);
    let mut quad = object::Object::new(object::Shape::create_rectangle(400.0,400.0));
    while window.running{
        window.get_events();
        if window.input.is_key_pressed(glutin::VirtualKeyCode::O){
           quad.set_opacity(0.2);
        }
        if window.input.is_key_released(glutin::VirtualKeyCode::A){
            quad.set_color([0.5,0.0,0.0]);
        }
        window.render(&mut quad);
       // window.render();
        window.display();
    }
    println!("Hello, world!");
}

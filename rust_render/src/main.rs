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

fn main() {
    let mut window = window::Window::new("Test Screen",800,600,false,false);
    while window.running{
        window.get_events();
        if window.input.is_key_pressed(glutin::VirtualKeyCode::A){
            println!("A is pressed");
        }
        if window.input.is_key_released(glutin::VirtualKeyCode::A){
            println!("A is realeased");
        }

        window.render();
    }
    println!("Hello, world!");
}

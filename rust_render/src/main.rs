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
    let mut window = window::Window::new("wtf",800,600,false,false);
    loop{
        //window.get_events();

        window.render();
    }
    println!("Hello, world!");
}

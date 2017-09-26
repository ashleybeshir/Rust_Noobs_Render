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

fn main() {
    let mut window = window::Window::new("test",800,600,false,false);
    loop{
        let events = window.get_events();
        /*events.poll_events(|glutin::Event::WindowEvent{window_id: _, event}|{
            use glutin::WindowEvent::*;
            match event {
                KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _)
                | Closed => println!("close"),

                _ => (),
            }
        } );*/
        window.render();
    }
    println!("Hello, world!");
}

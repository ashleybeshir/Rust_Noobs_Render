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

const WIDTH:u32 = 800;
const HEIGHT:u32 = 600;

fn main() {
    let mut window = window::Window::new("Test Screen",WIDTH,HEIGHT,false,false);
    let mut camera : Camera = Camera::new(WIDTH,HEIGHT);
    let mut quad = object::Object::new(object::Shape::create_cirle(100.0));
    quad.set_position(400.0,400.0);
    quad.set_color([0.3,0.3,0.3]);
    while window.running{
        window.get_events();

        if window.input.get_mouse_scroll() < 0.0 {
            let mut zoom = camera.get_scale();
            zoom -= 0.01;
            camera.set_scale(zoom);
        }else if window.input.get_mouse_scroll() > 0.0{
            let mut zoom = camera.get_scale();
            zoom += 0.01;
            camera.set_scale(zoom);
        }

        if window.input.is_key_released(glutin::VirtualKeyCode::A){
            camera.move_camera(50.0,0.0);
        }
        window.set_camera(&camera);
        window.render(&mut quad);

        window.display();
    }
    println!("Hello, world!");
}

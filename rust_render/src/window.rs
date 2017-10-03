
use glutin;
use glutin::GlContext;
use gfx_device_gl;
use renderer;
use input;
use object::Object;

pub struct Window {
    event_loop : glutin::EventsLoop,
    window     : glutin::GlWindow,
    renderer : renderer::Renderer,
    factory : gfx_device_gl::Factory,
    pub input : input::Input,
    pub running : bool
}

impl Window{
    //Creates and returns a window depending on given constructs
    pub fn new<T:Into<String>>(title : T,width : u32,height : u32,fullscreen : bool,vsync : bool)-> Self{
        let events_loop = glutin::EventsLoop::new();

        let builder = if fullscreen {
            glutin::WindowBuilder::new().with_fullscreen(glutin::get_primary_monitor())
        } else {
            glutin::WindowBuilder::new()
        };

        let builder = builder.clone()
            .with_dimensions(width, height)
            .with_title(title.into());

        let context = glutin::ContextBuilder::new()
            .with_vsync(vsync)
            .with_multisampling(0);

        let (renderer,window,factory) =  renderer::Renderer::new(builder,context,&events_loop);
        Window{event_loop : events_loop,window : window, renderer: renderer , input : input::Input::new(), running : true,factory : factory}

    }
    //Retrieves all events and sends them to input struct
    pub fn get_events(&mut self){
        let input = &mut self.input;
        let running = &mut self.running;

        self.window.swap_buffers().unwrap();
        input.clear();
        self.event_loop.poll_events(|event| {
            use glutin::WindowEvent::{Resized, Closed, KeyboardInput, MouseInput, MouseMoved, MouseWheel};
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {

                    Closed => *running = false,
                    KeyboardInput {
                        input : glutin::KeyboardInput{
                            state,
                            virtual_keycode : Some(keycode),..
                        },..
                    } => input.add_key(state,keycode),
                    MouseInput {
                        state,
                        button,..
                    } => input.add_button(state,button),
                    MouseWheel {
                        delta,..
                    } => input.add_mouse_wheel(delta),
                    MouseMoved {
                        position: (x,y),
                        ..
                    } => input.add_mouse_position(x as f32,y as f32),

                    _ => ()
                },
                _ => ()
            }
        });

    }
    pub fn render(&mut self,mut object : &Object)
    {
        if object.update == true{

        }
        self.renderer.render();
    }

}

use glutin;
use glutin::GlContext;
use renderer;


pub struct Window {
    event_loop : glutin::EventsLoop,
    window     : glutin::GlWindow,
    renderer : renderer::Renderer,

}

impl Window{

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

        let (renderer,window) =  renderer::Renderer::new(builder,context,&events_loop);
        Window{event_loop : events_loop,window : window, renderer: renderer}

    }
    pub fn get_events(&mut self)-> &mut glutin::EventsLoop{
        self.window.swap_buffers().unwrap();
        return &mut self.event_loop;
    }
    pub fn render(&mut self)
    {
        self.renderer.render();
    }

}

use glutin;
use gfx;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

struct Window {
    event_loop : glutin::EventsLoop,
    window     : glutin::GlWindow,
    device     : Device,

}

impl Window{
    //efffefeerffr
    pub fn new(x : u32,y : u32,name : &str)-> Window{
        let events_loop = glutin::EventsLoop::new();
        let builder = glutin::WindowBuilder::new()
            .with_title(name)
            .with_dimensions(x, y);
        let context = glutin::ContextBuilder::new();
        let (window, mut device, mut factory, main_color, mut main_depth) =
            gfx_glutin::init::<ColorFormat, DepthFormat>(builder, context,&events_loop);
    }
}

use gfx;
#[cfg(feature = "opengl")]
use gfx_device_gl;
#[cfg(feature = "opengl")]
use gfx_window_glutin;
#[cfg(feature = "opengl")]
use glutin;
use gfx::Device;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct Renderer{
    device: gfx_device_gl::Device,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    out_color: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
    out_depth: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
}
impl Renderer{
    #[cfg(feature = "opengl")]
    pub fn new(builder: glutin::WindowBuilder,context: glutin::ContextBuilder,event_loop: &glutin::EventsLoop) -> (Self,glutin::GlWindow){
        let (window, device, mut factory, color, depth) =
            gfx_window_glutin::init(builder, context, event_loop);

        let renderer = Renderer{
            device : device,
            encoder : factory.create_command_buffer().into(),
            out_color : color,
            out_depth : depth,
        };

        return (renderer,window);


    }
    pub fn render(&mut self)
    {

        self.encoder.clear(&self.out_color, [0.0, 0.0, 0.0, 1.0]);
        self.encoder.flush(&mut self.device);
        self.device.cleanup();
        //window.swap_buffers().unwrap();

    }

}
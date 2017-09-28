
use gfx;
#[cfg(feature = "opengl")]
use gfx_device_gl;
#[cfg(feature = "opengl")]
use gfx_window_glutin;
#[cfg(feature = "opengl")]
use glutin;
use gfx::Device;
use gfx::traits::FactoryExt;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
        uv: [f32;2] = "a_Uv",
    }
    constant Locals{
        color: [f32;4] = "l_Color",

    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "b_Locals",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
const SQUARE: [Vertex; 3] = [
    Vertex { pos: [0.5, -0.5], color: WHITE , uv: [0.0,0.0]},
    Vertex { pos: [-0.5, -0.5], color: WHITE , uv: [0.0,0.0]},
    Vertex { pos: [-0.5, 0.5], color: WHITE ,uv: [0.0,0.0]}
];
#[derive(Clone, Debug)]
pub struct GpuData{
    slice : gfx::Slice<gfx_device_gl::Resources>,
    vertices : gfx::handle::Buffer<gfx_device_gl::Resources,Vertex>,

}
pub struct Renderer{
    factory : gfx_device_gl::Factory,
    device: gfx_device_gl::Device,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    out_color: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
    out_depth: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
    basic_shape_pso : gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,

}
impl Renderer{
    #[cfg(feature = "opengl")]
    pub fn new(builder: glutin::WindowBuilder,context: glutin::ContextBuilder,event_loop: &glutin::EventsLoop) -> (Self,glutin::GlWindow){
        let (window, device, mut factory, color, depth) =
            gfx_window_glutin::init(builder, context, event_loop);
        let basic_pos = factory.create_pipeline_simple(
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader/basicshapev.glsl")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader/basicshapef.glsl")),
            pipe::new()
        ).unwrap();
        let renderer = Renderer{
            device : device,
            encoder : factory.create_command_buffer().into(),
            out_color : color,
            out_depth : depth,
            basic_shape_pso : basic_pos,
            factory : factory,
        };


        return (renderer,window);


    }
    pub fn render(&mut self)
    {
        /*let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&SQUARE, ());
        let mut data = pipe::Data {
            vbuf: vertex_buffer,
            out: self.out_color.clone(),
        };*/

        self.encoder.clear(&data.out, [0.0, 0.0, 0.0, 1.0]);
      //  self.encoder.draw(&slice,&self.basic_shape_pso,&data);
        self.encoder.flush(&mut self.device);
        self.device.cleanup();
        //window.swap_buffers().unwrap();

    }

}
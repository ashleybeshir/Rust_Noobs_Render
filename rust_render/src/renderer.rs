
use gfx;
#[cfg(feature = "opengl")]
use gfx_device_gl;
#[cfg(feature = "opengl")]
use gfx_window_glutin;
#[cfg(feature = "opengl")]
use glutin;
use gfx::Device;
use gfx::traits::FactoryExt;
use cgmath;
use object::Object;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32;2] = "a_Uv",
    }
    constant Locals{
        color: [f32;4] = "l_Color",
       /* uv_range: [f32;4] = "l_Uv",
        proj: [[f32;4];4] = "l_Pro",
        model: [[f32;4];4] = "l_Mod",
        view: [[f32;4];4] = "l_View",*/
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "b_Locals",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

#[derive(Clone, Debug)]
pub struct GpuData{
    pub slice : Option<gfx::Slice<gfx_device_gl::Resources>>,
    pub vertices : Option<gfx::handle::Buffer<gfx_device_gl::Resources,Vertex>>,
    pub constants: Option<gfx::handle::Buffer<gfx_device_gl::Resources, Locals>>,

}
pub struct Renderer{
    device: gfx_device_gl::Device,
    pub encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    pub out_color: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
    out_depth: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
    basic_shape_pso : gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    proj : cgmath::Matrix4<f32>,

}
impl Renderer{
    #[cfg(feature = "opengl")]
    pub fn new(builder: glutin::WindowBuilder,context: glutin::ContextBuilder,event_loop: &glutin::EventsLoop) -> (Self,glutin::GlWindow,gfx_device_gl::Factory){
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
            proj : cgmath::ortho(0.0,800.0,600.0,0.0,1.0,100.0),

        };


        return (renderer,window,factory);


    }
    pub fn render(&mut self,object : &mut Object)
    {
        //let model = cgmath::Matrix4::from_translation(cgmath::Vector3(0.0,0.0,1.0));
        let constant = object.gpudata.constants.as_ref().unwrap();
        let vertices = object.gpudata.vertices.as_ref().unwrap();
        let slice = object.gpudata.slice.as_ref().unwrap();
        self.encoder.update_constant_buffer(constant,&Locals{color:object.get_color()});
        let data = pipe::Data{
            vbuf : vertices.clone(),
            locals : constant.clone(),
            out : self.out_color.clone(),
        };
       // self.encoder.clear(&data.out, );
        self.encoder.draw(slice,&self.basic_shape_pso,&data);



    }
    pub fn display(&mut self)
    {
        self.encoder.flush(&mut self.device);
        self.device.cleanup();
    }

}
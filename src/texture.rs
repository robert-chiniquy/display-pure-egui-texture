use luminance::backend::color_slot::ColorSlot;
use luminance::backend::depth_slot::DepthSlot;
use luminance::framebuffer::Framebuffer;
#[allow(unused_imports)]
use luminance::pixel::{NormRGB8UI, SRGBA8UI};
use luminance::tess::Mode;
use luminance::texture::{Dimensionable, GenMipmaps, MinFilter, Sampler};
use luminance::UniformInterface;
use luminance::{
    blending::{Blending, Equation, Factor},
    context::GraphicsContext,
    pipeline::{PipelineState, TextureBinding},
    pixel::NormUnsigned,
    render_state::RenderState,
    shader::Uniform,
    texture::Dim2,
};
use luminance_front::*;

const VS: &str = include_str!("shaders/texture-vs.glsl");
const FS: &str = include_str!("shaders/texture-fs.glsl");

#[derive(UniformInterface)]
struct ShaderInterface {
    tex: Uniform<TextureBinding<Dim2, NormUnsigned>>,
}

pub struct RenderTexture {}

impl RenderTexture {
    pub fn render<C, B, D, CS, DS>(
        surface: &mut C,
        back_buffer: &Framebuffer<Backend, D, CS, DS>,
        texels: &[u8],
        size: [u32; 2],
    ) -> Result<luminance::pipeline::Render<luminance::pipeline::PipelineError>, String>
    where
        C: GraphicsContext<Backend = Backend>,
        D: Dimensionable,
        CS: ColorSlot<Backend, D>,
        DS: DepthSlot<Backend, D>,
    {
        // backface culling is off by default
        let render_st = RenderState::default()
            .set_blending(Blending {
                equation: Equation::Additive,
                src: Factor::One,
                dst: Factor::SrcAlphaComplement,
            })
            .set_depth_test(None);

        let pipeline_st = PipelineState::default()
            .enable_srgb(true)
            .set_clear_color([0.6, 0.6, 0.6, 1.]);

        let building_program = surface
            .new_shader_program::<(), (), ShaderInterface>()
            .from_strings(VS, None, None, FS);

        let built_program = match building_program {
            Ok(p) => p,
            Err(_e) => {
                // log!("{:?}", _e);
                // println!("{:?}", _e);
                panic!("Can't build program");
            }
        };

        let mut program = built_program.ignore_warnings();

        // Can't easily parameterize the Pixel type
        // due to https://github.com/phaazon/luminance-rs/issues/477
        let mut tex = surface
            .new_texture::<Dim2, SRGBA8UI>(
                size,
                0,
                Sampler {
                    min_filter: MinFilter::Linear,
                    ..Sampler::default()
                },
            )
            .expect("texture creation");

        tex.upload_raw(GenMipmaps::No, texels)
            .expect("texture upload");

        let tess = surface
            .new_tess()
            .set_vertex_nb(4)
            .set_mode(Mode::TriangleFan)
            .build()
            .unwrap();

        let render = surface
            .new_pipeline_gate()
            .pipeline(&back_buffer, &pipeline_st, |pipeline, mut shd_gate| {
                let bound_tex = pipeline.bind_texture(&mut tex)?;

                shd_gate.shade(&mut program, |mut iface, uni, mut rdr_gate| {
                    iface.set(&uni.tex, bound_tex.binding());

                    rdr_gate.render(&render_st, |mut tess_gate| tess_gate.render(&tess))
                })
            })
            .assume();

        Ok(render)
    }
}

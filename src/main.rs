// use std::path::PathBuf;
use std::{thread, time};

use glfw::Context as _;
use luminance::pixel::NormRGB8UI;
use luminance::texture::Dim2;
use luminance_front::Backend;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};

use trypure::texture::RenderTexture;
use trypure::ui;

fn main() {
    /*
    let path = PathBuf::from("./source.jpg");
    let texture = image::open(&path)
        .map(|img| img.flipv().to_rgb8())
        .unwrap_or_else(|_| panic!("image {}", path.display()));
    let texels = texture.as_raw();
    */

    /*
    let mut texels: Vec<u8> = vec![];
    for i in 0..(800 * 800 * 3) as i32 {
        texels.push(((i % 17) * 14) as u8);
    }
    */

    let (texels, size) = ui::load_egui_texels();

    let dim = WindowDim::Windowed {
        width: size[0],
        height: size[1],
    };

    let mut surface =
        GlfwSurface::new_gl33("try", WindowOpt::default().set_dim(dim)).expect("GLFW surface");

    let back_buffer = surface.context.back_buffer().unwrap();

    match RenderTexture::render::<_, Backend, Dim2, (), (), NormRGB8UI>(
        &mut surface.context,
        &back_buffer,
        &texels,
        size,
    ) {
        Ok(render) => {
            surface.context.window.glfw.poll_events();
            if render.is_ok() {
                surface.context.window.swap_buffers();
            }
        }

        Err(e) => println!("{:?}", e),
    }

    let twenty_seconds = time::Duration::from_secs(20);
    thread::sleep(twenty_seconds);
}

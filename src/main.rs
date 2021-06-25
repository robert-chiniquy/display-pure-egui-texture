// #![cfg(not(target_arch = "wasm32"))]
#![allow(unused_imports)]
// use std::path::PathBuf;
use std::{thread, time};

#[cfg(not(target_arch = "wasm32"))]
use glfw::Context as _;
#[allow(unused_imports)]
use luminance::pixel::{NormRGB8UI, SRGBA8UI};
use luminance::texture::Dim2;
#[cfg(not(target_arch = "wasm32"))]
use luminance_front::Backend;
#[cfg(not(target_arch = "wasm32"))]
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};

use trypure::texture::RenderTexture;
use trypure::ui;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Depending on the texture you need to flip the pixel type for texture creation in texture.rs
    // Can't easily parameterize the Pixel type
    // due to https://github.com/phaazon/luminance-rs/issues/477

    /*
    //NormRGB8UI
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

    // NormRGB8UI
    // let (texels, size) = ui::load_egui_texels_rgb_only();

    // SRGBA8UI
    let (texels, size) = ui::load_egui_texels();

    let dim = WindowDim::Windowed {
        width: size[0],
        height: size[1],
    };

    let mut surface = GlfwSurface::new_gl33("Egui texture", WindowOpt::default().set_dim(dim))
        .expect("GLFW surface");

    let back_buffer = surface.context.back_buffer().unwrap();

    match RenderTexture::render::<_, Backend, Dim2, (), ()>(
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

    let one_minute = time::Duration::from_secs(60);
    thread::sleep(one_minute);
}

#[cfg(target_arch = "wasm32")]
pub fn main() {}

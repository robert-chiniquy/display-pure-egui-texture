#[allow(unused_imports)]
use crate::ui;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use luminance_web_sys::WebSysWebGL2Surface;

#[cfg(target_arch = "wasm32")]
use crate::texture::RenderTexture;

#[cfg(target_arch = "wasm32")]
const CANVAS: &str = "canvas";

#[allow(unused_macros)]
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    use luminance::{pixel::SRGBA8UI, texture::Dim2};
    use luminance_front::Backend;

    // SRGBA8UI
    let (texels, size) = ui::load_egui_texels();

    //
    /*
    let mut texels: Vec<u8> = vec![];
    for i in 0..(800 * 800 * 3) as i32 {
        texels.push(((i % 17) * 14) as u8);
    }
    let size = [800, 800];
    */

    let mut surface = WebSysWebGL2Surface::new(CANVAS).expect("web-sys surface");

    let back_buffer = surface.back_buffer().unwrap();

    match RenderTexture::render::<_, Backend, Dim2, (), (), SRGBA8UI>(
        &mut surface,
        &back_buffer,
        &texels,
        size,
    ) {
        Ok(_) => log!("rendered texture"),
        Err(e) => log!("{:?}", e),
    }
}

#[cfg(target_arch = "wasm32")]
use crate::texture::RenderTexture;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
const CANVAS: &str = "canvas";

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    let _w = WebSysWebGL2Surface::new(CANVAS).expect("web-sys surface");

    let _r: RenderTexture = RenderTexture {};
}

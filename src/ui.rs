// use egui::epaint::Texture as EguiTexture;
use egui::{CtxRef, RawInput};

pub fn load_egui_texels() -> (Vec<u8>, [u32; 2]) {
    let i = RawInput {
        pixels_per_point: Some(2.0),
        ..RawInput::default()
    };

    let mut egui_ctx = CtxRef::default();

    egui_ctx.begin_frame(i);

    let egui_texture = egui_ctx.texture();
    let egui_texture_size = [
        egui_ctx.texture().width as u32,
        egui_ctx.texture().height as u32,
    ];

    let (_, _) = egui_ctx.end_frame();

    let mut texels: Vec<u8> = vec![];

    for srgba in egui_texture.srgba_pixels() {
        texels.push(srgba.r());
        texels.push(srgba.g());
        texels.push(srgba.b());
        texels.push(srgba.a());
    }

    texels.reverse();

    (texels, egui_texture_size)
}

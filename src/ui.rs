// use egui::epaint::Texture as EguiTexture;
use egui::{CtxRef, RawInput};

pub fn load_egui_texels() -> (Vec<u8>, [u32; 2]) {
    let i = RawInput {
        pixels_per_point: Some(2.0),
        ..RawInput::default()
    };

    let mut ctx = CtxRef::default();

    ctx.begin_frame(i);

    egui::SidePanel::left("❤").show(&ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("hello");
        });
        ui.separator();
    });

    let egui_texture = ctx.texture();
    let egui_texture_size = [ctx.texture().width as u32, ctx.texture().height as u32];

    let (_, _) = ctx.end_frame();

    let mut texels: Vec<u8> = vec![];

    for srgba in egui_texture.srgba_pixels() {
        texels.push(srgba.r());
        texels.push(srgba.g());
        texels.push(srgba.b());
        texels.push(srgba.a());

        // println!("{:?}", srgba);
    }

    (texels, egui_texture_size)
}

pub fn load_egui_texels_rgb_only() -> (Vec<u8>, [u32; 2]) {
    let i = RawInput {
        pixels_per_point: Some(2.0),
        ..RawInput::default()
    };

    let mut ctx = CtxRef::default();

    ctx.begin_frame(i);

    egui::SidePanel::left("❤").show(&ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("hello");
        });
        ui.separator();
    });

    let egui_texture = ctx.texture();
    let egui_texture_size = [ctx.texture().width as u32, ctx.texture().height as u32];

    let (_, _) = ctx.end_frame();

    let mut texels: Vec<u8> = vec![];

    for srgba in egui_texture.srgba_pixels() {
        texels.push(srgba.r());
        texels.push(srgba.g());
        texels.push(srgba.b());
        texels.push(srgba.a());

        // println!("{:?}", srgba);
    }

    (texels, egui_texture_size)
}

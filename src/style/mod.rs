use eframe::egui::{ColorImage, Context, TextureHandle};

pub struct WeaverStyle {
    pub del_btn: TextureHandle,
}

impl WeaverStyle {
    pub fn create(ctx: &Context) -> Self {
        Self {
            del_btn: ctx.load_texture(
                "del_btn",
                load_image_from_memory(include_bytes!("../res/light_del_btn.png")).unwrap(),
            ),
        }
    }
}

fn load_image_from_memory(image_data: &[u8]) -> Result<ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}

pub const DEL_BTN_SIZE: f32 = 12.0;

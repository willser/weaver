use crate::color;
use eframe::egui::style::{WidgetVisuals, Widgets};
use eframe::egui::{
    ColorImage, Context, FontId, FontSelection, Rounding, Stroke, TextureHandle, Ui,
};

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

pub fn get_widgets(expansion: f32) -> Widgets {
    Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: color::BLACK,
            bg_stroke: Stroke {
                width: 1.0,
                color: color::BLACK,
            },
            rounding: Rounding::none(),
            fg_stroke: Stroke {
                width: 1.0,
                color: color::BLACK,
            },
            expansion,
        },
        inactive: WidgetVisuals {
            bg_fill: color::WHITE,
            bg_stroke: Stroke {
                width: 1.0,
                color: color::GRAY,
            },
            rounding: Rounding::none(),
            fg_stroke: Stroke {
                width: 1.0,
                color: color::BLACK,
            },
            expansion,
        },
        hovered: WidgetVisuals {
            bg_fill: color::WHITE,
            bg_stroke: Stroke {
                width: 1.0,
                color: color::LIGHT_SKY_BLUE,
            },
            rounding: Rounding::none(),
            fg_stroke: Stroke {
                width: 1.0,
                color: color::BLACK,
            },
            expansion,
        },
        active: WidgetVisuals {
            bg_fill: color::WHITE,
            bg_stroke: Stroke {
                width: 1.0,
                color: color::WHITE,
            },
            rounding: Rounding::none(),
            fg_stroke: Stroke {
                width: 1.0,
                color: color::BLACK,
            },
            expansion,
        },
        open: WidgetVisuals {
            bg_fill: color::WHITE,
            bg_stroke: Stroke {
                width: 1.0,
                color: color::LIGHT_SKY_BLUE,
            },
            rounding: Rounding::none(),
            fg_stroke: Stroke {
                width: 1.0,
                color: color::BLACK,
            },
            expansion,
        },
    }
}

pub fn get_row_height(ui: &mut Ui) -> (FontId, f32) {
    let id = FontSelection::default().resolve(ui.style());
    let row_height = ui.fonts().row_height(&id);
    (id, row_height)
}

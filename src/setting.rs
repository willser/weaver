use crate::Weaver;
use eframe::egui::{Context, DragValue, FontDefinitions, Grid, Window};
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;
// use eframe::egui::{FontData, FontDefinitions, FontFamily};
//use font_loader::system_fonts;
//use std::collections::BTreeMap;

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub font_size: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { font_size: 20.0 }
    }
}

pub fn draw_settings_window(weaver: &mut Weaver, ctx: &Context) {
    Window::new("Settings")
        .resizable(false)
        .open(&mut weaver.show_settings)
        .collapsible(false)
        .show(ctx, |ui| {
            Grid::new("setting_grid")
                .min_col_width(200.00)
                .num_columns(2)
                // .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Font Size");
                    ui.add(
                        DragValue::new(&mut weaver.settings.font_size)
                            .clamp_range(RangeInclusive::new(15, 40))
                            .max_decimals(0)
                            .speed(0.5),
                    );
                    ui.end_row();
                });
        });
}

#[cfg(windows)]
pub fn get_default_font() -> FontDefinitions {
    FontDefinitions::default()
}

#[cfg(unix)]
pub fn get_default_font() -> FontDefinitions {
    FontDefinitions::default()
}

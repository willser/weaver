use eframe::egui;
use eframe::egui::style::TextStyle::{Body, Button, Heading, Small};
use eframe::egui::FontFamily::Proportional;
use eframe::egui::{
    ComboBox, Context, DragValue, FontData, FontDefinitions, FontFamily, FontId, Grid, Window,
};
use eframe::epi::egui::style::TextStyle::Monospace;
use font_loader::system_fonts::FontPropertyBuilder;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    #[serde(skip)]
    pub show_settings: bool,
    pub font_size: f32,
    /// TODO Change to String
    // #[serde(skip)]
    pub font: String,
    #[serde(skip)]
    pub system_font: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_settings: false,
            font_size: 20.0,
            font: "".to_string(),
            system_font: vec![],
        }
    }
}

impl Settings {
    pub fn set(&mut self, ctx: &Context) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(self.font_size * 1.5, Proportional)),
            (Body, FontId::new(self.font_size, Proportional)),
            (Monospace, FontId::new(self.font_size, Proportional)),
            (Button, FontId::new(self.font_size, Proportional)),
            (Small, FontId::new(self.font_size * 0.75, Proportional)),
        ]
        .into();
        ctx.set_style(style);
    }

    pub fn draw_settings_window(&mut self, ctx: &Context) {
        Window::new("Settings")
            .resizable(false)
            .open(&mut self.show_settings)
            .collapsible(false)
            .show(ctx, |ui| {
                Grid::new("setting_grid")
                    .min_col_width(200.00)
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Font Size");
                        ui.add(
                            DragValue::new(&mut self.font_size)
                                .clamp_range(RangeInclusive::new(15, 40))
                                .max_decimals(0)
                                .speed(0.5),
                        );
                        ui.end_row();
                        ui.label("Font");

                        ComboBox::from_id_source("font_comboBox")
                            .selected_text(format!("{:?}", self.font))
                            .show_ui(ui, |ui| {
                                for x in &self.system_font {
                                    ui.selectable_value(&mut self.font, x.clone(), x);
                                }
                            });

                        ui.end_row();
                    });
            });
    }

    pub fn local_settings(&mut self, ctx: &egui::Context) {
        // Init font first.
        self.check_init_font();
        ctx.set_fonts(get_font(&self.font));
    }

    fn check_init_font(&mut self) {
        let font = match &self.font {
            None => get_default_font(),
            Some(font) => font.to_owned(),
        };

        // Check this system has this font.
        let vec = font_loader::system_fonts::query_all();

        if vec.contains(&font) {
            self.font = font;
        } else {
            self.font = "".to_string();
        }
        self.system_font = vec;
    }
}

fn get_font(font: &String) -> FontDefinitions {
    if font.is_empty() {
        FontDefinitions::default()
    } else {
        match font_loader::system_fonts::get(&FontPropertyBuilder::new().family(font).build()) {
            None => FontDefinitions::default(),
            Some((font_vec, _)) => {
                let mut fonts = FontDefinitions::default();
                fonts
                    .font_data
                    .insert(font.to_string(), FontData::from_owned(font_vec));
                fonts
                    .families
                    .get_mut(&FontFamily::Proportional)
                    .unwrap()
                    .insert(0, font.to_string());
                fonts
                    .families
                    .get_mut(&FontFamily::Monospace)
                    .unwrap()
                    .push(font.to_string());
                fonts
            }
        }
    }
}

#[cfg(windows)]
pub fn get_default_font() -> String {
    "Microsoft YaHei UI".to_string()
}

#[cfg(unix)]
pub fn get_default_font() -> String {
    ""
}

#[test]
fn all_font() {
    // use rust_fontconfig::{FcFontCache, FcPattern};
    // let cache = FcFontCache::build();
    // let result = cache.query(&FcPattern {
    //     name: Some(String::from("Microsoft YaHei UI")),
    //     ..Default::default()
    // });
    //
    // println!("font path: {:?}", result);
}

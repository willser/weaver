use eframe::egui;
use eframe::egui::style::TextStyle::{Body, Button, Heading, Small};
use eframe::egui::FontFamily::Proportional;
use eframe::egui::{
    ComboBox, Context, DragValue, FontData, FontDefinitions, FontFamily, FontId, Grid, Window,
};
use eframe::epi::egui::style::TextStyle::Monospace;
use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    #[serde(skip)]
    pub show_settings: bool,
    pub font_size: f32,
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
                    .min_col_width(100.00)
                    .num_columns(2)
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
                            .selected_text(format!("{}", self.font))
                            .width(self.font_size * 10.0)
                            .show_ui(ui, |ui| {
                                for x in &self.system_font {
                                    ui.selectable_value(&mut self.font, x.clone(), x.as_str());
                                }
                            });
                        ui.end_row();
                    });
            });
    }

    pub fn local_settings(&mut self, ctx: &egui::Context) {
        // Init font first.
        self.check_init_font();
        ctx.set_fonts(get_font(self.font.clone()));
    }

    fn check_init_font(&mut self) {
        let font = if self.font.is_empty() {
            get_default_font()
        } else {
            self.font.clone()
        };

        // Check current system has this font or not.
        match SystemSource::new().all_families() {
            Ok(mut families) => {
                if families.contains(&font) {
                    self.font = font;
                } else {
                    self.font = "".to_string();
                }

                // None font select
                families.insert(0, "NO SETTING".to_string());

                self.system_font = families;
            }
            Err(_error) => {}
        };
    }
}

fn get_font(font_family: String) -> FontDefinitions {
    // TODO handle error
    let mut fonts = FontDefinitions::default();
    if !font_family.is_empty() {
        let name = FamilyName::Title(font_family.clone());
        let properties = Properties::default();
        match SystemSource::new().select_best_match(&[name], &properties) {
            Ok(handle) => {
                let vec = match handle {
                    Handle::Path { path, .. } => {
                        // TODO Better impl
                        FontData::from_owned(match std::fs::read(path) {
                            Ok(vec) => vec,
                            Err(_err) => return fonts,
                        })
                    }
                    Handle::Memory { bytes, .. } => FontData::from_owned(bytes.to_vec()),
                };

                fonts.font_data.insert(font_family.clone(), vec);
                fonts
                    .families
                    .get_mut(&FontFamily::Proportional)
                    .unwrap()
                    .insert(0, font_family.clone());
                fonts
                    .families
                    .get_mut(&FontFamily::Monospace)
                    .unwrap()
                    .push(font_family);
            }
            Err(_err) => {}
        };
    }
    fonts
}

#[cfg(windows)]
pub fn get_default_font() -> String {
    if let Some((_, lang)) = locale_config::Locale::current().tags().next() {
        let lang_str = lang.to_string();
        match lang_str.as_str() {
            "zh-CN" => {
                return "Microsoft YaHei UI".to_string();
            }
            _ => {}
        }
    }
    return "".to_string();
}

#[cfg(unix)]
pub fn get_default_font() -> String {
    ""
}

use eframe::egui::FontDefinitions;
// use eframe::egui::{FontData, FontDefinitions, FontFamily};
//use font_loader::system_fonts;
//use std::collections::BTreeMap;

#[cfg(windows)]
pub fn get_default_font() -> FontDefinitions {
    FontDefinitions::default()
}

#[cfg(unix)]
pub fn get_default_font() -> FontDefinitions {
    FontDefinitions::default()
}

#![feature(vec_retain_mut)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod request;
mod setting;

use crate::request::Request;
use eframe::egui::{Color32, RichText, TextStyle, WidgetText};
use eframe::{egui, epi};
use request::http::Http;
use serde::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
pub struct Weaver {
    requests: Vec<Http>,
    active: usize,
}

/// Unused for now
// #[derive(Deserialize, Serialize)]
// enum Request {
//     HTTP(Http),
// }

impl Default for Weaver {
    fn default() -> Self {
        Self {
            requests: vec![],
            active: 0,
        }
    }
}

impl epi::App for Weaver {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        // let Self { requests } = self;
        // TODO styles
        // ctx.set_style()

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("New", |ui| {
                    if ui.button("Http").clicked() {
                        self.requests.insert(0, Http::default());
                    }
                });

                #[cfg(debug_assertions)]
                ui.menu_button("Dev", |ui| {
                    if ui.button("Remove All").clicked() {
                        self.requests.clear();
                    }
                });
            });
        });
        egui::SidePanel::left("request_list").show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading(RichText::from("REQUEST").text_style(TextStyle::Heading));
            });

            // TODO Change to `selectable_value`
            // This variable used for save `active request` when click.
            // Worry about out of bounds is unnecessary,because active will be set to 0 after remove any `request`.Someday maybe change this implementation.
            let mut index: usize = 0;
            self.requests.retain(|request| {
                let is_active = self.active == index;

                // TODO Better styles.
                let widget_text = WidgetText::from(request.request_name()).color(if is_active {
                    Color32::BLUE
                } else {
                    Color32::GRAY
                });

                let request_button = ui.button(widget_text);
                if request_button.clicked() {
                    // self.active = index
                    self.active = index;
                }

                // Only active request can be deleted
                let deleted = is_active && request_button.secondary_clicked();
                if deleted {
                    self.active = 0
                }
                index += 1;
                return !deleted;
            });

            // ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     *value += 1.0;
            // }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("will", "https://github.com/willser");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.requests.get_mut(self.active) {
            None => {}
            Some(request) => {
                request.view(ui);
            }
        });
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &epi::Frame,
        storage: Option<&dyn epi::Storage>,
    ) {
        ctx.set_fonts(setting::get_default_font());
        // Load previous app state (if any).
        if let Some(storage) = storage {
            // TODO change key in feature
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        // TODO change key in feature
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str {
        "weaver"
    }
}

fn main() {
    let app = Weaver::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

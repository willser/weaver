#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod http;

use eframe::{egui, epi};
use http::Http;
use serde::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
pub struct Weaver {
    requests: Vec<Http>,
}

/// Unused for now
#[derive(Deserialize, Serialize)]
enum Request {
    HTTP(Http),
}

impl Default for Weaver {
    fn default() -> Self {
        Self { requests: vec![] }
    }
}

impl epi::App for Weaver {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        // let Self { requests } = self;

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
            ui.heading("Request");

            for request in &self.requests {
                if ui.button(&request.name).clicked() {}
            }

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

        egui::CentralPanel::default().show(ctx, |_ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.heading("eframe template");
            // ui.hyperlink("https://github.com/emilk/eframe_template");
            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/master/",
            //     "Source code."
            // ));
            // egui::warn_if_debug_build(ui);
        });
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
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

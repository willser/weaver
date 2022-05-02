// #![feature(vec_retain_mut)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod color;
mod components;
mod request;
mod setting;

use crate::egui::Direction;
use crate::request::Request;
use crate::setting::Settings;
use eframe::egui::{Align, Button, CentralPanel, Layout, ScrollArea};
use eframe::{egui, epi};
use request::http::Http;
use serde::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
pub struct Weaver {
    requests: Vec<Http>,
    active: usize,
    settings: Settings,
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
            settings: Default::default(),
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
        self.settings.set(ctx);
        self.settings.draw_settings_window(ctx);

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

                ui.menu_button("Settings", |_ui| self.settings.show_settings = true);
            });
        });
        egui::SidePanel::left("request_list")
            .max_width(300.0)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    // TODO Change to `selectable_value`
                    // This variable used for save `active request` when click.
                    // Worry about out of bounds is unnecessary,because active will be set to 0 after remove any `request`.Someday maybe change this implementation.
                    let mut index: usize = 0;
                    self.requests.retain(|request| {
                        let is_active = self.active == index;
                        ui.horizontal(|ui| {
                            let request_button = ui
                                .with_layout(
                                    Layout::from_main_dir_and_cross_align(
                                        Direction::TopDown,
                                        Align::Min,
                                    )
                                    .with_main_wrap(false)
                                    .with_cross_justify(true),
                                    |ui| {
                                        ui.add(Button::new(request.request_name()).fill(
                                            if is_active {
                                                color::LIGHT_SKY_BLUE
                                            } else {
                                                color::GRAY
                                            },
                                        ))
                                    },
                                )
                                .inner;

                            if request_button.clicked() {
                                self.active = index;
                            }
                            if request_button.hovered() && self.active == index {
                                egui::show_tooltip_text(
                                    ui.ctx(),
                                    egui::Id::new("delete_tip"),
                                    "Right click to delete",
                                );
                            }

                            // Only active request can be deleted
                            let deleted = is_active && request_button.secondary_clicked();
                            if deleted {
                                // TODO double check
                                self.active = 0
                            }
                            index += 1;
                            !deleted
                        })
                        .inner
                    });
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("Powered by ");
                        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                        ui.label(" and ");
                        ui.hyperlink_to("will", "https://github.com/willser");
                    });
                });
            });

        CentralPanel::default().show(ctx, |ui| match self.requests.get_mut(self.active) {
            None => {}
            Some(request) => {
                ScrollArea::vertical().show(ui, |ui| {
                    request.view(ui);
                });
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
        // Load previous app state (if any).
        if let Some(storage) = storage {
            // TODO change key in feature
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
        }
        // Init font after load data from local
        self.settings.local_settings(ctx);
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

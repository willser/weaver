// #![feature(vec_retain_mut)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod color;
mod components;
mod curl;
mod request;
mod setting;
mod style;

use crate::curl::Curl;
use crate::egui::Direction;
use crate::request::{ClickType, Request};
use crate::setting::Settings;
use crate::style::WeaverStyle;
use eframe::egui::{CentralPanel, ScrollArea, Style, Visuals};
use eframe::{egui, App, Frame, Storage};
use request::http::Http;
use serde::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
pub struct Weaver {
    requests: Vec<Http>,
    active: usize,
    settings: Settings,
    #[serde(skip)]
    curl: Curl,
    // TODO Make it out of `Weaver` struct.Use lazy_static maybe better.
    #[serde(skip)]
    style: Option<WeaverStyle>,
}

impl Default for Weaver {
    fn default() -> Self {
        Self {
            requests: vec![],
            active: 0,
            settings: Default::default(),
            curl: Default::default(),
            style: None,
        }
    }
}

impl App for Weaver {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // let Self { requests } = self;
        // TODO styles
        // ctx.set_style()
        self.settings.set(ctx);
        self.settings.draw_settings_window(ctx);
        self.curl.draw_curl_window(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("New", |ui| {
                    if ui.button("Http").clicked() {
                        self.requests.insert(0, Http::default());
                    }
                    if ui.button("From cURL").clicked() {
                        self.curl.show_curl_window = true
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
            .width_range(100.0..=300.0)
            .show(ctx, |ui| {
                ui.add_space(2.0);
                ScrollArea::vertical().show(ui, |ui| {
                    // TODO Change to `selectable_value`
                    // This variable used for save `active request` when click.
                    // Worry about out of bounds is unnecessary,because active will be set to 0 after remove any `request`.Someday maybe change this implementation.
                    let mut index: usize = 0;
                    let style = &self.style.as_ref().unwrap();
                    self.requests.retain(|request| {
                        let mut result = true;

                        match request.request_name_view(self.active == index, ui, style) {
                            ClickType::Click => {
                                self.active = index;
                            }
                            ClickType::Delete => result = false,
                            _ => {}
                        };
                        index += 1;
                        result
                    });
                });

                // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                //     ui.horizontal(|ui| {
                //         ui.spacing_mut().item_spacing.x = 0.0;
                //         ui.label("Powered by ");
                //         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                //         ui.label(" and ");
                //         ui.hyperlink_to("will", "https://github.com/willser");
                //     });
                // });
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

    fn save(&mut self, storage: &mut dyn Storage) {
        // TODO change key in feature
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "weaver",
        native_options,
        Box::new(|creation_context| {
            let mut weaver: Weaver = match creation_context.storage {
                None => Default::default(),
                Some(storage) => eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default(),
            };
            let context = &creation_context.egui_ctx;
            let mut visuals = Visuals::light();
            visuals.widgets.open.rounding = Default::default();
            visuals.widgets.hovered.rounding = Default::default();
            visuals.widgets.active.rounding = Default::default();
            visuals.widgets.noninteractive.rounding = Default::default();
            visuals.widgets.inactive.rounding = Default::default();
            visuals.window_rounding = Default::default();
            visuals.window_shadow = Default::default();
            context.set_style(Style {
                override_text_style: None,
                override_font_id: None,
                text_styles: Default::default(),
                wrap: None,
                spacing: Default::default(),
                interaction: Default::default(),
                visuals,
                animation_time: 0.0,
                debug: Default::default(),
                explanation_tooltips: false,
            });
            weaver.settings.local_settings(context);
            weaver.style = Some(WeaverStyle::create(context));
            Box::new(weaver)
        }),
    );
}

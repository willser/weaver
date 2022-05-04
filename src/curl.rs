use crate::color;
use clap::{self, Arg, Command, Parser};
use eframe::egui::{Button, Context, ScrollArea, TextEdit, Window};
use shellwords::MismatchedQuotes;

#[derive(Default)]
pub struct Curl {
    pub show_curl_window: bool,
    error: Option<String>,
    text: String,
}

#[derive(Parser, Debug)]
struct CurlInfo {
    #[clap(short, long, multiple_values = true)]
    header: Vec<String>,
}

impl Curl {
    pub fn draw_curl_window(&mut self, ctx: &Context) {
        Window::new("Import from cURL")
            .resizable(true)
            .open(&mut self.show_curl_window)
            .collapsible(false)
            .show(ctx, |ui| {
                if let Some(_) = self.error {
                    ui.style_mut().visuals.extreme_bg_color = color::CRIMSON
                }
                // ui.style_mut().visuals.selection.stroke.color = color::CRIMSON;
                ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    ui.add(TextEdit::multiline(&mut self.text).desired_width(f32::INFINITY))
                });

                ui.horizontal(|ui| {
                    if ui.add(Button::new("Import")).clicked() {
                        {
                            let vec = match shellwords::split(self.text.as_str()) {
                                Ok(vec) => vec,
                                Err(error) => {
                                    println!("{}", error);
                                    vec![]
                                }
                            };
                            // TODO finish this
                        }
                    }
                })
            });
    }
}

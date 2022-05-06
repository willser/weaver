use crate::color;
use eframe::egui;
use eframe::egui::epaint::text::TextWrapping;
use eframe::egui::style::Margin;
use eframe::egui::style::TextStyle::Body;
use eframe::egui::text::LayoutJob;
use eframe::egui::{Frame, InnerResponse, Label, Sense, Stroke, Ui, Vec2};

///TODO  Remove pub in future.
pub mod http;

/// Request trait
pub(crate) trait Request {
    fn request_name(&self) -> &str;

    fn view(&mut self, ui: &mut Ui);

    // TODO until ws,graphQL or rpc be supported
    // fn request_type(&self) -> String;

    fn request_name_view(&self, is_active: bool, ui: &mut Ui) -> InnerResponse<ClickType> {
        ui.style_mut().spacing.item_spacing = Vec2::new(0., 0.);
        ui.horizontal(|ui| {
            Frame {
                inner_margin: Margin::same(5.),
                outer_margin: Margin::same(0.),
                fill: color::WHITE,
                stroke: Stroke {
                    width: 1.0,
                    color: color::BLACK,
                },
                ..Frame::default()
            }
            .show(ui, |ui| {
                ui.set_height(36.);
                let max_width = ui.available_width();
                ui.set_width(max_width);
                ui.set_max_width(max_width);

                ui.style_mut().wrap = Some(true);

                let font_id = match ui.style().text_styles.get(&Body) {
                    None => Default::default(),
                    Some(font_id) => font_id.clone(),
                };

                let mut job = LayoutJob::simple(
                    self.request_name().to_string(),
                    font_id,
                    color::BLACK,
                    max_width,
                );
                job.wrap = TextWrapping {
                    max_rows: 1,
                    break_anywhere: true,
                    max_width,
                    overflow_character: Some('…'),
                };

                let request_button = ui.add(Label::new(job).sense(Sense::click()));
                if request_button.clicked() {
                    return ClickType::Click;
                }
                if request_button.hovered() && is_active {
                    egui::show_tooltip_text(
                        ui.ctx(),
                        egui::Id::new("delete_tip"),
                        "Right click to delete",
                    );
                }

                // Only active request can be deleted
                if is_active && request_button.secondary_clicked() {
                    ClickType::Delete
                } else {
                    ClickType::None
                }
            })
        })
        .inner
    }
}

pub(crate) enum ClickType {
    None,
    Delete,
    Click,
}

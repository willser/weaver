use crate::color;
use crate::components::Frame;
use eframe::egui;
use eframe::egui::epaint::text::TextWrapping;
use eframe::egui::style::Margin;
use eframe::egui::style::TextStyle::Body;
use eframe::egui::text::LayoutJob;
use eframe::egui::{Label, Rounding, Sense, Stroke, Ui, Vec2};

///TODO  Remove pub in future.
pub mod http;

/// Request trait
pub(crate) trait Request {
    fn request_name(&self) -> &str;

    fn view(&mut self, ui: &mut Ui);

    // TODO until ws,graphQL or rpc be supported
    // fn request_type(&self) -> String;

    fn request_name_view(&self, is_active: bool, ui: &mut Ui) -> ClickType {
        ui.style_mut().spacing.item_spacing = Vec2::new(0., 0.);
        let response = Frame {
            inner_margin: Margin::same(10.),
            outer_margin: Margin::same(0.),
            fill: if is_active {
                color::LIGHT_SKY_BLUE
            } else {
                color::WHITE
            },
            stroke: Stroke {
                width: 1.0,
                color: color::BLACK,
            },
            sense: Sense::click(),
            ..Frame::default()
        }
        .show(ui, |ui| {
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

            ui.add(Label::new(job));
        });

        let frame_response = response.response;

        let rect = frame_response.rect;

        if ui.is_rect_visible(rect) {
            if frame_response.clicked() {
                return ClickType::Click;
            }
            if frame_response.hovered() && is_active {
                egui::show_tooltip_text(
                    ui.ctx(),
                    egui::Id::new("delete_tip"),
                    "Right click to delete",
                );
            }
            if frame_response.hovered() {
                ui.painter_at(rect).rect_stroke(
                    rect,
                    Rounding::none(),
                    Stroke::new(2., color::BLACK),
                );
            }

            // Only active request can be deleted
            if is_active && frame_response.secondary_clicked() {
                return ClickType::Delete;
            }
        }
        ClickType::None
    }
}

pub(crate) enum ClickType {
    None,
    Delete,
    Click,
}

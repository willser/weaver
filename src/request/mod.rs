use crate::components::Frame;
use crate::egui::Color32;
use crate::style::DEL_BTN_SIZE;
use crate::{color, WeaverStyle};
use eframe::egui;
use eframe::egui::epaint::text::TextWrapping;
use eframe::egui::style::Margin;
use eframe::egui::style::TextStyle::Body;
use eframe::egui::text::LayoutJob;
use eframe::egui::{Id, ImageButton, Label, Layout, Pos2, Rect, Rounding, Sense, Stroke, Ui, Vec2};

///TODO  Remove pub in future.
pub mod http;

/// Request trait
pub(crate) trait Request {
    fn request_name(&self) -> &str;

    fn view(&mut self, ui: &mut Ui);

    // TODO until ws,graphQL or rpc be supported
    // fn request_type(&self) -> String;

    fn request_name_view(
        &self,
        is_active: bool,
        ui: &mut Ui,
        weaver_style: &WeaverStyle,
    ) -> ClickType {
        ui.style_mut().spacing.item_spacing = Vec2::new(0., 0.);
        ui.horizontal(|ui| {
            let response = Frame {
                inner_margin: Margin {
                    left: 5.0,
                    right: DEL_BTN_SIZE / 2.0,
                    top: 10.0,
                    bottom: 10.0,
                },
                fill: if is_active {
                    color::LIGHT_SKY_BLUE
                } else {
                    color::WHITE
                },
                outer_margin: Margin::same(0.),
                sense: Sense::click(),
                ..Frame::default()
            }
            .show(ui, |ui| {
                let max_width = ui.available_width();
                ui.set_width(max_width - DEL_BTN_SIZE);
                ui.set_max_width(max_width - DEL_BTN_SIZE);

                ui.style_mut().wrap = Some(true);

                let font_id = match ui.style().text_styles.get(&Body) {
                    None => Default::default(),
                    Some(font_id) => font_id.clone(),
                };

                let mut job = LayoutJob::simple(
                    self.request_name().to_string(),
                    font_id,
                    color::BLACK,
                    max_width - DEL_BTN_SIZE,
                );
                job.wrap = TextWrapping {
                    max_rows: 1,
                    break_anywhere: true,
                    max_width: max_width - DEL_BTN_SIZE,
                    overflow_character: Some('…'),
                };

                ui.add(Label::new(job));
            });

            let btn_response = ui.with_layout(Layout::right_to_left(), |ui| {
                Frame {
                    inner_margin: Margin::same(0.0),
                    outer_margin: Margin::same(0.0),
                    fill: get_bg_color(is_active),
                    ..Frame::default()
                }
                .show(ui, |ui| {
                    let button =
                        ImageButton::new(weaver_style.del_btn.id(), Vec2::splat(DEL_BTN_SIZE))
                            .frame(true);

                    ui.style_mut().visuals.widgets.inactive.bg_fill = get_bg_color(is_active);
                    if ui.add(button).clicked() {
                        Some(ClickType::Delete)
                    } else {
                        None
                    }
                })
            });

            // Delete
            if let Some(click_type) = btn_response.inner.inner {
                return click_type;
            }
            let frame_response = response.response;
            let btn_rect = btn_response.response.rect;
            let label_rect = frame_response.rect;

            if ui.is_rect_visible(label_rect) {
                if frame_response.clicked() {
                    return ClickType::Click;
                }
                if frame_response.hovered() {
                    egui::show_tooltip_text(ui.ctx(), Id::new("name_tip"), self.request_name());
                    let rect = Rect {
                        min: Pos2 {
                            x: label_rect.min.x,
                            y: label_rect.min.y,
                        },
                        max: Pos2 {
                            x: btn_rect.max.x,
                            y: btn_rect.max.y,
                        },
                    };
                    ui.painter_at(rect).rect_stroke(
                        rect,
                        Rounding::none(),
                        Stroke::new(3., get_bg_color(true)),
                    );
                }
            }
            ClickType::None
        })
        .inner
    }
}

fn get_bg_color(is_active: bool) -> Color32 {
    if is_active {
        color::LIGHT_SKY_BLUE
    } else {
        color::WHITE
    }
}
pub(crate) enum ClickType {
    None,
    Delete,
    Click,
}

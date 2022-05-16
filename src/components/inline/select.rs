use crate::color;
use crate::components::Frame;
use crate::egui::epaint::text::TextWrapping;
use crate::egui::Rounding;
use crate::style::get_widgets;
use eframe::egui::style::{Margin, Widgets};
use eframe::egui::text::LayoutJob;
use eframe::egui::TextStyle::Body;
use eframe::egui::{
    popup, pos2, Align, FontSelection, Id, Label, Pos2, Rect, Response, Sense, TextStyle, Ui, Vec2,
    WidgetInfo, WidgetText, WidgetType,
};
use eframe::epaint;
use eframe::epaint::{RectShape, Shadow, Shape, Stroke};

pub struct Select<'t> {
    // Widgets of element
    pub(crate) value: &'t mut WidgetText,
    pub widgets: Widgets,
    pub outer_margin: Option<Margin>,
    pub inner_margin: Option<Margin>,
    pub id_source: Id,
    pub label: Option<WidgetText>,
    pub selected_text: WidgetText,
    pub option: Option<Vec<WidgetText>>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub override_text_size: Option<f32>,
}

impl<'t> Select<'t> {
    pub fn new(id: impl Into<Id>, value: &'t mut WidgetText) -> Self {
        Self {
            value,
            widgets: Default::default(),
            outer_margin: Default::default(),
            inner_margin: Default::default(),
            id_source: id.into(),
            label: None,
            selected_text: Default::default(),
            option: None,
            width: None,
            height: None,
            override_text_size: None,
        }
    }

    pub fn show(self, ui: &mut Ui) {
        let Self {
            value,
            widgets,
            outer_margin,
            inner_margin,
            id_source,
            label,
            selected_text,
            option,
            width,
            height,
            override_text_size,
        } = self;

        let font_id = match ui.style().text_styles.get(&Body) {
            None => Default::default(),
            Some(font_id) => font_id.clone(),
        };

        let popup_id = id_source.with("popup");
        let where_to_put_background = ui.painter().add(Shape::Noop);
        let inner_margin = if let Some(margin) = inner_margin {
            margin
        } else {
            Margin::same(0.0)
        };

        let wrap_width = ui.available_width();

        let mut text = if let Some(wid) = width {
            let mut job =
                LayoutJob::simple(value.text().to_string(), font_id.clone(), color::BLACK, wid);
            job.halign = Align::Center;
            job.wrap = TextWrapping {
                max_rows: 1,
                break_anywhere: true,
                max_width: wid,
                overflow_character: Some('…'),
            };
            WidgetText::LayoutJob(job).into_galley(ui, None, wrap_width, TextStyle::Body)
        } else {
            value
                .clone()
                .into_galley(ui, None, wrap_width, TextStyle::Body)
        };

        let mut desired_size = text.size() + inner_margin.sum();

        let (mut rect, response) = ui.allocate_exact_size(
            desired_size,
            Sense {
                click: false,
                drag: false,
                focusable: false,
            },
        );

        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, value.text()));

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            // TODO Temp solution,remove after there is inline:input
            let text_pos = ui
                .layout()
                .align_size_within_rect(text.size(), rect.shrink2(inner_margin.left_top()))
                .min;

            text.paint_with_visuals(ui.painter(), text_pos, visuals);

            let response = ui.interact(rect, id_source, Sense::click());

            let visuals = ui.style().interact(&response);

            ui.painter().set(
                where_to_put_background,
                RectShape {
                    rect: rect.expand(visuals.expansion),
                    rounding: visuals.rounding,
                    fill: visuals.bg_fill,
                    stroke: visuals.bg_stroke,
                },
            );

            if response.clicked() {
                ui.memory().toggle_popup(popup_id);
            }
            popup::popup_below_widget(ui, popup_id, &response, |ui| match option {
                None => {
                    ui.label("No option");
                }
                Some(options) => {
                    let width = ui.available_width();

                    for x in options {
                        // ui.vertical_centered(|ui| {
                        let is_selected = value.text() == x.text();

                        let (color, stroke) = if is_selected {
                            (color::LIGHT_SKY_BLUE, Stroke::new(1.0, color::BLACK))
                        } else {
                            (color::WHITE, Stroke::none())
                        };

                        let mut select_frame = Frame {
                            inner_margin: Margin::same(2.0),
                            outer_margin: Margin::same(0.0),
                            rounding: Rounding::none(),
                            shadow: Shadow::default(),
                            fill: color,
                            stroke,
                            sense: Sense::click(),
                        }
                        .show(ui, |ui| {
                            ui.set_width(width);
                            let mut job = LayoutJob::simple(
                                x.text().to_string(),
                                font_id.clone(),
                                color::BLACK,
                                width,
                            );
                            job.halign = Align::Center;
                            job.wrap = TextWrapping {
                                max_rows: 1,
                                break_anywhere: true,
                                max_width: width,
                                overflow_character: Some('…'),
                            };

                            ui.add(Label::new(job));
                        })
                        .response;

                        if select_frame.clicked() {
                            *value = x;
                            select_frame.mark_changed();
                        };

                        let label_rect = select_frame.rect;
                        if select_frame.hovered() {
                            ui.painter_at(label_rect).rect_stroke(
                                label_rect,
                                Rounding::none(),
                                Stroke::new(3., color::LIGHT_SKY_BLUE),
                            );
                        }
                        // });
                    }
                }
            });
        }
    }
}

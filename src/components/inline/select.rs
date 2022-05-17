use crate::color;
use crate::egui::Rounding;
use crate::style::get_widgets;
use eframe::egui::style::{Margin, Widgets};
use eframe::egui::{
    popup, pos2, FontSelection, Id, Label, Rect, Response, Sense, TextStyle, Ui, Vec2, WidgetInfo,
    WidgetText, WidgetType,
};
use eframe::epaint;
use eframe::epaint::{RectShape, Shape, Stroke};

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

        let popup_id = id_source.with("popup");
        let where_to_put_background = ui.painter().add(Shape::Noop);
        let inner_margin = if let Some(margin) = inner_margin {
            margin
        } else {
            Margin::same(0.0)
        };

        let wrap_width = ui.available_width();

        let text = value
            .clone()
            .into_galley(ui, None, wrap_width, TextStyle::Body);

        let mut desired_size = text.size() + inner_margin.sum();

        let (rect, response) = ui.allocate_exact_size(
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
                    rect,
                    rounding: visuals.rounding,
                    fill: visuals.bg_fill,
                    stroke: visuals.bg_stroke,
                },
            );

            if response.clicked() {
                println!("{:?}", ui.next_widget_position());
                println!("{:?}", rect);
                println!("lasted : {:?}", text_pos);
                ui.memory().toggle_popup(popup_id);
            }
            let inner = popup::popup_below_widget(ui, popup_id, &response, |ui| match option {
                None => {
                    ui.label("No option");
                }
                Some(options) => {
                    for x in options {
                        let mut response = ui.selectable_label(value.text() == x.text(), x.text());
                        if response.clicked() {
                            *value = x;
                            response.mark_changed();
                        }
                    }
                }
            });
        }
    }
}

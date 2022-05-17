use eframe::egui::style::{Margin, Widgets};
use eframe::egui::{
    pos2, FontSelection, Id, Label, Rect, Response, Sense, TextStyle, Ui, Vec2, WidgetInfo,
    WidgetText, WidgetType,
};
use eframe::epaint;
use eframe::epaint::Shape;

pub struct Select {
    // Widgets of element
    pub widgets: Widgets,
    pub outer_margin: Option<Margin>,
    pub inner_margin: Option<Margin>,
    pub id_source: Id,
    pub label: Option<WidgetText>,
    pub selected_text: WidgetText,
    pub option: Option<WidgetText>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub override_text_size: Option<f32>,
}

impl Select {
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
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

    pub fn show(&self, ui: &mut Ui) -> Response {
        let Self {
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

        let button_id = ui.make_persistent_id(id_source);

        let outer_margin = if let Some(margin) = outer_margin {
            *margin
        } else {
            Margin::same(0.0)
        };
        let inner_margin = if let Some(margin) = inner_margin {
            *margin
        } else {
            Margin::same(0.0)
        };

        let row_height = if let Some(text_size) = override_text_size {
            *text_size
        } else {
            let id = FontSelection::default().resolve(ui.style());
            ui.fonts().row_height(&id)
        };

        let wrap_width = ui.available_width();

        let text = match label {
            None => WidgetText::from(""),
            Some(text) => text.clone(),
        };

        let text = text.into_galley(ui, None, wrap_width, TextStyle::Body);

        let mut desired_size = text.size() + inner_margin.sum();

        let (rect, response) = ui.allocate_exact_size(
            desired_size,
            Sense {
                click: false,
                drag: false,
                focusable: false,
            },
        );

        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, text.text()));

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let text_pos = ui
                .layout()
                .align_size_within_rect(text.size(), rect.expand2(inner_margin.sum()))
                .min;

            text.paint_with_visuals(ui.painter(), text_pos, visuals);
        }
        response
    }
}

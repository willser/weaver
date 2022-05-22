mod frame;

pub use frame::Frame;

use crate::egui::Response;
use eframe::egui::{Id, Rect, Sense, Ui, Vec2, Widget};

pub fn widget_with_size(ui: &mut Ui, size: Vec2, widget: impl Widget) -> Response {
    let widget_rect = Rect::from_min_size(
        ui.min_rect().max + Vec2::new(5.0, ui.min_rect().min.y - ui.min_rect().max.y),
        size,
    );
    ui.put(widget_rect, widget)
}

pub fn close_button(ui: &mut Ui, rect: Rect, id: Id) -> Response {
    let response = ui.interact(rect, id, Sense::click());
    ui.expand_to_include_rect(response.rect);

    let visuals = ui.style().interact(&response);
    let rect = rect.shrink(2.0).expand(visuals.expansion);
    let stroke = visuals.fg_stroke;
    ui.painter() // paints \
        .line_segment([rect.left_top(), rect.right_bottom()], stroke);
    ui.painter() // paints /
        .line_segment([rect.right_top(), rect.left_bottom()], stroke);
    response
}

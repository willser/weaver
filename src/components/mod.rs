mod frame;

pub use frame::Frame;

use crate::egui::Response;
use eframe::egui::{Rect, Ui, Vec2, Widget};

pub fn widget_with_size(ui: &mut Ui, size: Vec2, widget: impl Widget) -> Response {
    let widget_rect = Rect::from_min_size(
        ui.min_rect().max + Vec2::new(5.0, ui.min_rect().min.y - ui.min_rect().max.y),
        size,
    );
    ui.put(widget_rect, widget)
}

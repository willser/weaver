use eframe::egui::Ui;

///TODO  Remove pub in future.
pub mod http;

/// Request trait
pub(crate) trait Request {
    fn request_name(&self) -> &str;

    fn view(&mut self, ui: &mut Ui);
}

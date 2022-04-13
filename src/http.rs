// use crate::http::Method::{GET, POST};
use eframe::egui::{Align, ComboBox, Label, Layout, Ui};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Http {
    //Research this field is necessary or not
    id: String,
    // remove pub when it's unnecessary
    pub name: String,
    url: String,
    method: Method,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            id: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect(),
            name: "New http request".to_string(),
            url: "".to_string(),
            method: Default::default(),
        }
    }
}

impl Http {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::left_to_right().with_cross_align(Align::Min), |ui| {
            ui.add(Label::new("REQUEST NAME: ").wrap(true));
            ui.text_edit_singleline(&mut self.name);
        });

        ui.with_layout(Layout::left_to_right().with_cross_align(Align::Min), |ui| {
            // ui.horizontal(|ui| {
            //     ui.selectable_value(&mut self.method, Method::GET, "GET");
            //     ui.selectable_value(&mut self.method, Method::POST, "POST");
            // });

            ComboBox::from_id_source("comboBox")
                .selected_text(format!("{:?}", self.method))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.method, Method::GET, "GET");
                    ui.selectable_value(&mut self.method, Method::POST, "POST");
                });

            ui.text_edit_singleline(&mut self.url);
            let _ = ui.button("SEND");
        });
    }
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
enum Method {
    POST,
    GET,
}

impl Default for Method {
    fn default() -> Self {
        Self::GET
    }
}

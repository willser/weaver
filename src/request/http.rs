use std::ffi::OsStr;
use std::path::PathBuf;
// use crate::http::Method::{GET, POST};
use crate::request::Request;
use eframe::egui::{Align, ComboBox, Label, Layout, Ui};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Http {
    //Research this field is necessary or not
    id: String,
    name: String,
    url: String,
    method: Method,
    header: Vec<(String, String)>,
    text_param: String,
    form_param: Vec<(String, String, Option<PathBuf>, FormParamType)>,
    param_type: ParamType,
    show_header: bool,
}

// impl PartialEq for Http {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
// }

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
enum FormParamType {
    File,
    Text,
}

impl Default for FormParamType {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Deserialize, Serialize)]
enum ParamType {
    FormData,
    Raw,
}

impl Default for ParamType {
    fn default() -> Self {
        Self::Raw
    }
}

impl Default for Http {
    fn default() -> Self {
        Self {
            id: get_uuid(),
            name: "New http request".to_string(),
            url: "".to_string(),
            method: Default::default(),
            header: vec![],
            text_param: "".to_string(),
            form_param: vec![],
            param_type: Default::default(),
            show_header: true,
        }
    }
}

fn get_uuid() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

impl Request for Http {
    fn request_name(&self) -> &String {
        &self.name
    }

    fn view(&mut self, ui: &mut Ui) {
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

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.show_header, true, "HEADER");
            ui.selectable_value(&mut self.show_header, false, "PARAM");
        });
        ui.separator();

        match self.show_header {
            true => {
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.vertical_centered(|ui| {
                            self.header.retain_mut(|(key, value)| {
                                ui.horizontal(|ui| {
                                    ui.text_edit_singleline(key);
                                    ui.text_edit_singleline(value);
                                    !ui.button("Del").clicked()
                                })
                                .inner
                            });
                            ui.horizontal(|ui| {
                                ui.vertical_centered(|ui| {
                                    if ui.button("Add").clicked() {
                                        self.header.push(("".to_string(), "".to_string()));
                                    }
                                })
                            });
                        });
                    })
                });
            }
            false => {
                // TODO
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.vertical_centered(|ui| {
                            let mut label = 0;
                            self.form_param.retain_mut(
                                |(key, value, path_buf, form_param_type)| {
                                    ui.horizontal(|ui| {
                                        ui.text_edit_singleline(key);
                                        // let string = get_uuid();
                                        label += 1;
                                        ComboBox::from_id_source(
                                            label.to_string() + "form_param_type_combo_box",
                                        )
                                        .selected_text(format!("{:?}", form_param_type))
                                        .show_ui(
                                            ui,
                                            |ui| {
                                                ui.selectable_value(
                                                    form_param_type,
                                                    FormParamType::Text,
                                                    "Text",
                                                );
                                                ui.selectable_value(
                                                    form_param_type,
                                                    FormParamType::File,
                                                    "File",
                                                );
                                            },
                                        );

                                        match form_param_type {
                                            FormParamType::File => {
                                                let file_button = ui.button(match path_buf {
                                                    Some(name) => name
                                                        .file_name()
                                                        .unwrap_or_else(|| OsStr::new("Open file…"))
                                                        .to_str()
                                                        .unwrap_or("Open file…"),
                                                    _ => "Open file…",
                                                });
                                                if file_button.clicked() {
                                                    if let Some(path) =
                                                        rfd::FileDialog::new().pick_file()
                                                    {
                                                        *path_buf = Some(path);
                                                    }
                                                }

                                                if file_button.secondary_clicked() {
                                                    *path_buf = None;
                                                }
                                            }
                                            FormParamType::Text => {
                                                ui.text_edit_singleline(value);
                                            }
                                        }

                                        !ui.button("Del").clicked()
                                    })
                                    .inner
                                },
                            );
                            ui.horizontal(|ui| {
                                ui.vertical_centered(|ui| {
                                    if ui.button("Add").clicked() {
                                        self.form_param.push((
                                            "".to_string(),
                                            "".to_string(),
                                            None,
                                            FormParamType::Text,
                                        ));
                                    }
                                })
                            });
                        });
                    })
                });
            }
        }
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

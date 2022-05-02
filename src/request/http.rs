use crate::request::Request;
use crate::{color, components};

use crate::egui::Vec2;
use eframe::egui::{
    Align, Button, CollapsingHeader, ComboBox, Layout, ScrollArea, TextEdit, Ui, WidgetText,
};
use poll_promise::Promise;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::blocking::multipart;
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::PathBuf;

type RequestResult = Result<Response, String>;

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
    // TODO Discuss this structs' impl
    #[serde(skip)]
    result: Option<RequestResult>,
    // TODO add error handle
    #[serde(skip)]
    state: Option<Promise<RequestResult>>,
}

#[derive(Clone)]
struct Response {
    body: String,
    size: Option<u64>,
    code: StatusCode,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
enum FormParamType {
    File,
    Text,
}

impl Default for FormParamType {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Clone, Copy)]
enum ParamType {
    None,
    FormData,
    Json,
    Query,
    Other,
}

impl Default for ParamType {
    fn default() -> Self {
        Self::Json
    }
}

impl ParamType {
    fn get_content_type(&self) -> String {
        match self {
            ParamType::FormData => "multipart/form-data".to_string(),
            ParamType::Json => "application/json".to_string(),
            _ => "".to_string(),
        }
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
            result: Option::default(),
            state: Option::default(),
        }
    }
}
//
// enum Result {
//     Error(String),
//     Ok(Response),
// }

fn get_uuid() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

impl Request for Http {
    fn request_name(&self) -> &str {
        if self.name.is_empty() {
            return "Http Request";
        }
        self.name.as_str()
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::left_to_right().with_cross_align(Align::Min), |ui| {
            // ui.add(Label::new("REQUEST NAME: ").wrap(true));
            TextEdit::singleline(&mut self.name)
                .desired_width(f32::INFINITY)
                .show(ui);
        });
        ui.add_space(15.0);
        ui.with_layout(Layout::left_to_right().with_cross_align(Align::Min), |ui| {
            ComboBox::from_id_source("comboBox")
                .selected_text(format!("{:?}", self.method))
                .show_ui(ui, |ui| {
                    self.method_select(ui);
                });

            TextEdit::singleline(&mut self.url)
                .desired_width(ui.available_width() - 100.0)
                .show(ui);

            // Button::new("SEND");

            match &self.state {
                None => {
                    // TODO width of button https://github.com/emilk/egui/blob/master/egui_demo_lib/src/demo/tests.rs

                    let send_button = components::widget_with_size(
                        ui,
                        Vec2::new(80.0, 18.0),
                        Button::new(WidgetText::from("SEND").color(color::WHITE))
                            .fill(color::DODER_BLUE),
                    );

                    if send_button.clicked() {
                        match Url::parse(&self.url) {
                            Ok(url) => {
                                self.state = Some(get_request_promise(
                                    self.method.clone(),
                                    self.param_type,
                                    url,
                                    self.header.clone(),
                                    self.text_param.clone(),
                                    self.form_param.clone(),
                                ));
                            }
                            Err(err) => {
                                self.state = Some(Promise::from_ready(Err(err.to_string())))
                            }
                        };
                    };
                }
                Some(promise) => {
                    // Cancel the request

                    match promise.ready() {
                        None => {
                            if components::widget_with_size(
                                ui,
                                Vec2::new(80.0, 18.0),
                                Button::new(WidgetText::from("CANCEL").color(color::WHITE))
                                    .fill(color::CRIMSON),
                            )
                            .clicked()
                            {
                                self.state = None;
                            }
                        }
                        Some(result) => {
                            self.result = Some(result.clone());
                            self.state = None;
                        }
                    }
                }
            }
        });
        if let Some(Result::Err(error_text)) = &self.result {
            ui.add_space(15.0);
            ScrollArea::vertical().show(ui, |ui| {
                let mut error_text = error_text.as_str();
                ui.add(
                    eframe::egui::TextEdit::multiline(&mut error_text) // for cursor height
                        .text_color(color::CRIMSON)
                        .desired_rows(1)
                        .desired_width(f32::INFINITY),
                );
            });
        }
        ui.add_space(15.0);
        CollapsingHeader::new("Request")
            .default_open(true)
            .show(ui, |ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.show_header, true, "HEADER");
                    ui.selectable_value(&mut self.show_header, false, "PARAM");
                });
                ui.add_space(5.0);
                match self.show_header {
                    true => {
                        if !self.header.is_empty() {
                            ui.group(|ui| {
                                let col_width = (ui.available_width() - 70.0) / 2.0;
                                self.header.retain_mut(|(key, value)| {
                                    !ui.horizontal(|ui| {
                                        ui.add(TextEdit::singleline(key).desired_width(col_width));
                                        ui.add(
                                            TextEdit::singleline(value).desired_width(col_width),
                                        );
                                        ui.add(
                                            Button::new(
                                                WidgetText::from("DEL").color(color::WHITE),
                                            )
                                            .fill(color::CRIMSON),
                                        )
                                    })
                                    .inner
                                    .clicked()
                                });
                                ui.add_space(5.0);
                                ui.horizontal(|ui| {
                                    ui.vertical_centered(|ui| {
                                        if ui.button("Add").clicked() {
                                            self.header.push(("".to_string(), "".to_string()));
                                        }
                                    })
                                });
                            });
                        }
                    }
                    false => {
                        self.param_view(ui);
                    }
                }
            });

        ui.add_space(15.0);
        CollapsingHeader::new("Response")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(Result::Ok(response)) = &self.result {
                    ui.label(format!(
                        "{} {} {}",
                        response.code.as_str(),
                        response.code.canonical_reason().unwrap_or(""),
                        match response.size {
                            None => {
                                "".to_string()
                            }
                            Some(size) => {
                                format!(" ,Size: {}", size)
                            }
                        }
                    ));

                    ui.horizontal(|ui| {
                        ui.group(|ui| {
                            ui.vertical_centered_justified(|ui| {
                                ui.add_enabled_ui(true, |ui| {
                                    let mut response_body = response.body.as_str();
                                    ui.text_edit_multiline(&mut response_body)
                                });
                            })
                        })
                    });
                }
            });
    }
}

impl Http {
    fn param_type_view(&mut self, ui: &mut Ui) {
        match self.method {
            Method::Get => {
                ui.vertical_centered(|ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        self.param_type = ParamType::Query;
                        ui.selectable_value(&mut self.param_type, ParamType::Query, "query");
                    })
                });
            }
            _ => {
                ui.vertical_centered(|ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        ui.selectable_value(&mut self.param_type, ParamType::Json, "json");
                        ui.selectable_value(&mut self.param_type, ParamType::FormData, "form-data");
                        ui.selectable_value(&mut self.param_type, ParamType::None, "none");
                        ui.selectable_value(&mut self.param_type, ParamType::Other, "other");
                    })
                });
            }
        }
    }

    fn param_view(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(5.0);
                    self.param_type_view(ui);
                    ui.add_space(5.0);

                    match self.param_type {
                        ParamType::FormData => {
                            self.form_data_param_view(ui);
                        }
                        ParamType::Json => {
                            self.raw_param_view(ui);
                        }
                        ParamType::Other => {
                            self.raw_param_view(ui);
                        }
                        ParamType::Query => {
                            self.query_param_view(ui);
                        }
                        _ => {}
                    }
                });
            })
        });
    }

    fn form_data_param_view(&mut self, ui: &mut Ui) {
        let mut label = 0;
        let col_width = (ui.available_width() - 180.0) / 2.0;
        self.form_param
            .retain_mut(|(key, value, path_buf, form_param_type)| {
                ui.horizontal(|ui| {
                    !ui.horizontal(|ui| {
                        ui.add(TextEdit::singleline(key).desired_width(col_width));

                        label += 1;
                        // TODO center
                        ComboBox::from_id_source(label.to_string() + "form_param_type_combo_box")
                            .selected_text(format!("{:?}", form_param_type))
                            .width(70.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(form_param_type, FormParamType::Text, "Text");
                                ui.selectable_value(form_param_type, FormParamType::File, "File");
                            });

                        match form_param_type {
                            FormParamType::File => {
                                let file_button = components::widget_with_size(
                                    ui,
                                    Vec2::new(col_width, 18.0),
                                    Button::new(match path_buf {
                                        Some(name) => name
                                            .file_name()
                                            .unwrap_or_else(|| OsStr::new("Open file…"))
                                            .to_str()
                                            .unwrap_or("Open file…"),
                                        _ => "Open file…",
                                    }),
                                );
                                if file_button.clicked() {
                                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                                        *path_buf = Some(path);
                                    }
                                }

                                if file_button.secondary_clicked() {
                                    *path_buf = None;
                                }
                            }
                            FormParamType::Text => {
                                components::widget_with_size(
                                    ui,
                                    Vec2::new(col_width, 18.0),
                                    TextEdit::singleline(value),
                                );
                            }
                        }

                        components::widget_with_size(
                            ui,
                            Vec2::new(70.0, 18.0),
                            Button::new(WidgetText::from("DEL").color(color::WHITE))
                                .fill(color::CRIMSON),
                        )
                    })
                    .inner
                    .clicked()
                })
                .inner
            });
        ui.add_space(5.0);
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
    }

    fn raw_param_view(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.vertical_centered_justified(|ui| {
                ui.text_edit_multiline(&mut self.text_param);
            })
        });
    }

    fn query_param_view(&mut self, ui: &mut Ui) {
        let col_width = (ui.available_width() - 70.0) / 2.0;
        self.form_param.retain_mut(|(key, value, ..)| {
            !ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(key).desired_width(col_width));
                ui.add(TextEdit::singleline(value).desired_width(col_width));
                ui.add(
                    Button::new(WidgetText::from("DEL").color(color::WHITE)).fill(color::CRIMSON),
                )
            })
            .inner
            .clicked()
        });
        ui.add_space(5.0);
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
    }

    fn method_select(&mut self, ui: &mut Ui) {
        // TODO Better impl
        if ui
            .selectable_value(&mut self.method, Method::Get, "Get")
            .changed()
        {
            self.form_param = vec![];
        };
        if ui
            .selectable_value(&mut self.method, Method::Post, "Post")
            .changed()
        {
            self.form_param = vec![];
            self.param_type = ParamType::FormData;
        };
        if ui
            .selectable_value(&mut self.method, Method::Put, "Put")
            .changed()
        {
            self.form_param = vec![];
            self.param_type = ParamType::FormData;
        };
        if ui
            .selectable_value(&mut self.method, Method::Delete, "Delete")
            .changed()
        {
            self.form_param = vec![];
            self.param_type = ParamType::FormData;
        };
        if ui
            .selectable_value(&mut self.method, Method::Patch, "Patch")
            .changed()
        {
            self.form_param = vec![];
            self.param_type = ParamType::FormData;
        };
    }
}

/// Create a request promise by request information
fn get_request_promise(
    method: Method,
    param_type: ParamType,
    url: Url,
    headers: Vec<(String, String)>,
    text_param: String,
    form_param: Vec<(String, String, Option<PathBuf>, FormParamType)>,
) -> Promise<RequestResult> {
    Promise::spawn_thread(
        String::from("slow_operation"),
        // TODO More method request
        move || -> RequestResult {
            let client = reqwest::blocking::Client::new();

            let mut builder = match method {
                Method::Get => client.get(url),
                Method::Post => client.post(url),
                Method::Delete => client.delete(url),
                Method::Put => client.put(url),
                Method::Patch => client.patch(url),
            };
            for (k, v) in headers {
                builder = builder.header(k, v);
            }
            builder = match param_type {
                ParamType::FormData => {
                    let mut form = multipart::Form::new();
                    for (k, v_text, v_file, typ) in form_param {
                        match (typ, v_file) {
                            (FormParamType::File, Some(v_file)) => {
                                form = match form.file(k, v_file) {
                                    Ok(file) => file,
                                    Err(err) => return Err(format!("{}", err)),
                                };
                            }
                            (FormParamType::Text, _) => {
                                form = form.text(k, v_text);
                            }
                            _ => {}
                        }
                    }
                    builder.multipart(form)
                }
                ParamType::Json => builder.body(text_param),
                ParamType::Other => builder.body(text_param),
                ParamType::Query => builder.query(
                    &form_param
                        .iter()
                        .map(|(k, v, ..)| (k, v))
                        .collect::<Vec<(&String, &String)>>(),
                ),
                _ => builder,
            };

            // Override content-type if not empty.Maybe add a `override` button for user to select override or not.
            let content_type = param_type.get_content_type();
            if !content_type.is_empty() {
                builder = builder.header("Content-Type", content_type);
            }

            // .query(&query_param)
            let result = builder.send();

            return match result {
                Ok(result) => Result::Ok(Response {
                    code: result.status(),
                    size: result.content_length(),
                    body: result.text().unwrap_or_else(|_| "".to_string()),
                }),
                Err(err) => Err(format!("{}", err)),
            };
        },
    )
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
enum Method {
    Post,
    Get,
    Put,
    Delete,
    Patch,
}

impl Default for Method {
    fn default() -> Self {
        Self::Get
    }
}

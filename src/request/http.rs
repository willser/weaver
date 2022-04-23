use crate::request::Request;
use crate::Color32;
use eframe::egui::{Align, CollapsingHeader, ComboBox, Layout, ScrollArea, TextEdit, Ui};
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
    code: StatusCode,
}

// impl PartialEq for Http {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
// }

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
    FormData,
    Json,
    Query,
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
            ParamType::Query => "".to_string(),
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

        if self.name.len() > 15 {
            return &self.name[0..15];
        }

        &self.name.as_str()
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::left_to_right().with_cross_align(Align::Min), |ui| {
            // ui.add(Label::new("REQUEST NAME: ").wrap(true));
            ui.text_edit_singleline(&mut self.name);
        });

        ui.with_layout(Layout::left_to_right().with_cross_align(Align::Min), |ui| {
            ComboBox::from_id_source("comboBox")
                .selected_text(format!("{:?}", self.method))
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(&mut self.method, Method::GET, "GET")
                        .changed()
                    {
                        self.form_param = vec![];
                    };
                    if ui
                        .selectable_value(&mut self.method, Method::POST, "POST")
                        .changed()
                    {
                        self.param_type = ParamType::FormData;
                    };
                });

            ui.add(TextEdit::singleline(&mut self.url));

            match &self.state {
                None => {
                    let send_button = ui.button("SEND");
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
                            if ui.button("CANCEL").clicked() {
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
            // ui.horizontal(|ui| ui.label("Error"));

            ScrollArea::vertical().show(ui, |ui| {
                let mut error_text = error_text.as_str();
                ui.add(
                    eframe::egui::TextEdit::multiline(&mut error_text) // for cursor height
                        .text_color(Color32::RED)
                        // .desired_rows(10)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY),
                );
            });
        }

        CollapsingHeader::new("Request")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.show_header, true, "HEADER");
                    ui.selectable_value(&mut self.show_header, false, "PARAM");
                });

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
                        self.param_view(ui);
                    }
                }
            });

        CollapsingHeader::new("Response")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(Result::Ok(response)) = &self.result {
                    ui.label(format!(
                        "{} {}",
                        response.code.as_str(),
                        response.code.canonical_reason().unwrap_or("")
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
            Method::POST => {
                ui.vertical_centered(|ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        ui.selectable_value(&mut self.param_type, ParamType::Json, "json");
                        ui.selectable_value(&mut self.param_type, ParamType::FormData, "form-data");
                    })
                });
            }
            Method::GET => {
                ui.vertical_centered(|ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        self.param_type = ParamType::Query;
                        ui.selectable_value(&mut self.param_type, ParamType::Query, "query");
                    })
                });
            }
        }
    }

    fn param_view(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    self.param_type_view(ui);

                    match self.param_type {
                        ParamType::FormData => {
                            self.form_data_param_view(ui);
                        }
                        ParamType::Json => {
                            self.raw_param_view(ui);
                        }
                        ParamType::Query => {
                            self.query_param_view(ui);
                        }
                    }
                });
            })
        });
    }

    fn form_data_param_view(&mut self, ui: &mut Ui) {
        let mut label = 0;
        self.form_param
            .retain_mut(|(key, value, path_buf, form_param_type)| {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(key);
                    // let string = get_uuid();
                    label += 1;
                    ComboBox::from_id_source(label.to_string() + "form_param_type_combo_box")
                        .selected_text(format!("{:?}", form_param_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(form_param_type, FormParamType::Text, "Text");
                            ui.selectable_value(form_param_type, FormParamType::File, "File");
                        });

                    // TODO refactor following code
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
                                if let Some(path) = rfd::FileDialog::new().pick_file() {
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
            });
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
        self.form_param.retain_mut(|(key, value, ..)| {
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

            let request = match method {
                Method::GET => client.get(url),
                Method::POST => {
                    let mut builder = client.post(url);
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
                        _ => builder,
                    }
                    .header("Content-Type", param_type.get_content_type());

                    builder
                }
            };

            // .query(&query_param)
            let result = request.send();

            return match result {
                Ok(result) => Result::Ok(Response {
                    code: result.status(),
                    body: result.text().unwrap_or("".to_string()),
                }),
                Err(err) => Err(format!("{}", err)),
            };
        },
    )
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
enum Method {
    POST,
    GET,
}

impl Default for Method {
    fn default() -> Self {
        Self::GET
    }
}

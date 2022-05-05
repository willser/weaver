use crate::request::Request;
use crate::{color, components, style, Visuals};

use crate::egui::{FontSelection, Vec2};
use crate::style::get_row_height;
use eframe::egui;
use eframe::egui::text::LayoutJob;
use eframe::egui::{
    Align, Button, CollapsingHeader, ComboBox, FontId, Id, Layout, Pos2, Rect, Rounding,
    ScrollArea, Stroke, TextEdit, TextStyle, Ui, WidgetText,
};
use poll_promise::Promise;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::blocking::multipart;
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::ops::Add;
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
pub enum FormParamType {
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
        let (id, row_height) = crate::style::get_row_height(ui);
        ui.add_space(10.0);
        ui.with_layout(Layout::left_to_right().with_cross_align(Align::Min), |ui| {
            let mut pos2 = ui.next_widget_position();
            // TODO This value may be be decided by egui style,explore later
            pos2.y -= 2.5;

            let rect = Rect {
                min: pos2,
                max: Pos2 {
                    x: pos2.x + 5.0,
                    y: pos2.y + row_height + 14.0,
                },
            };

            ui.painter_at(rect).rect(
                rect,
                Rounding::none(),
                color::LIGHT_SKY_BLUE,
                Stroke::none(),
            );
            ui.add_space(15.0);
            ui.style_mut().visuals.widgets = crate::style::get_widgets(1.0);
            TextEdit::singleline(&mut self.name)
                .margin(Vec2::new(5.0, 5.0))
                .desired_width(ui.available_width() - 10.0)
                .show(ui);
        });
        ui.add_space(15.0);
        ui.with_layout(Layout::left_to_right().with_cross_align(Align::Min), |ui| {
            ui.style_mut().visuals.widgets = crate::style::get_widgets(5.0);
            ui.add_space(19.0);
            let mut job = LayoutJob::simple(
                format!("{:?}", &self.method),
                id.clone(),
                color::BLACK,
                50.0,
            );
            job.first_row_min_height = row_height + 2.0;
            ComboBox::from_id_source("comboBox")
                .selected_text(WidgetText::LayoutJob(job))
                .show_ui(ui, |ui| {
                    self.method_select(ui);
                });
            ui.add_space(10.0);
            ui.style_mut().visuals.widgets = crate::style::get_widgets(5.0);
            TextEdit::singleline(&mut self.url)
                .font(FontSelection::Style(TextStyle::Button))
                .desired_width(ui.available_width() - 110.0)
                .show(ui);

            // Button::new("SEND");
            ui.style_mut().visuals.widgets = crate::style::get_widgets(5.0);
            ui.add_space(5.0);

            // ui.with_layout(Layout::left_to_right().with_cross_align(Align::Max), |ui| {
            self.send_button(ui, id.clone(), row_height)
            // });
        });
        if let Some(Result::Err(error_text)) = &self.result {
            ui.add_space(15.0);
            let clear_btn_res = ui
                .vertical_centered(|ui| {
                    let mut error_text = error_text.as_str();

                    ui.horizontal(|ui| {
                        ui.add_space(15.0);

                        let next_pos = ui.next_widget_position();
                        let clear_btn_rect = Rect::from_min_max(
                            next_pos,
                            next_pos.add(Vec2::splat(row_height / 1.5)),
                        );
                        ui.add_space(row_height);
                        let clear_btn_res =
                            components::close_button(ui, clear_btn_rect, Id::new("clear_btn_rect"));

                        eframe::egui::TextEdit::multiline(&mut error_text) // for cursor height
                            .text_color(color::CRIMSON)
                            .desired_width(ui.available_width() - 25.0)
                            .desired_rows(1)
                            .show(ui);
                        clear_btn_res
                    })
                    .inner
                })
                .inner;

            if clear_btn_res.hovered() {
                egui::show_tooltip_text(ui.ctx(), Id::new("clear_btn_tooltip"), "Clear the error");
            }
            if clear_btn_res.clicked() {
                self.result = None
            }
        }
        ui.add_space(15.0);
        CollapsingHeader::new("Request")
            .default_open(true)
            .show(ui, |ui| {
                ui.add_space(5.0);
                ui.style_mut().visuals.widgets = crate::style::get_widgets(1.0);
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.show_header, true, "HEADER");
                    ui.selectable_value(&mut self.show_header, false, "PARAM");
                });
                ui.add_space(5.0);
                match self.show_header {
                    true => {
                        let group_rect = ui
                            .group(|ui| {
                                ui.set_width(ui.available_width());
                                ui.style_mut().visuals.widgets = Visuals::light().widgets;
                                ScrollArea::vertical()
                                    .max_height(ui.available_height() / 2.0)
                                    .show(ui, |ui| {
                                        ui.style_mut().visuals.widgets =
                                            crate::style::get_widgets(1.0);
                                        if !self.header.is_empty() {
                                            let col_width = (ui.available_width() - 70.0) / 2.0;
                                            let mut label = 0;
                                            self.header.retain_mut(|(key, value)| {
                                                ui.add_space(2.0);
                                                label += 1;
                                                !ui.horizontal(|ui| {
                                                    ui.add(
                                                        TextEdit::singleline(key)
                                                            .desired_width(col_width),
                                                    );
                                                    ui.add(
                                                        TextEdit::singleline(value)
                                                            .desired_width(col_width),
                                                    );

                                                    let clear_btn_rect =
                                                        Self::get_next_del_btn(row_height, ui);
                                                    ui.add_space(row_height);
                                                    components::close_button(
                                                        ui,
                                                        clear_btn_rect,
                                                        Id::new(
                                                            label.to_string()
                                                                + "remove_query_param_btn",
                                                        ),
                                                    )
                                                })
                                                .inner
                                                .clicked()
                                            });
                                        }
                                        ui.add_space(5.0);
                                        ui.horizontal(|ui| {
                                            ui.vertical_centered(|ui| {
                                                ui.style_mut().visuals.widgets.hovered.expansion =
                                                    2.0;
                                                let next_pos = ui.next_widget_position();
                                                let clear_btn_rect = Rect::from_min_max(
                                                    next_pos,
                                                    next_pos.add(Vec2::splat(row_height / 1.5)),
                                                );
                                                if components::add_button(
                                                    ui,
                                                    clear_btn_rect,
                                                    Id::new("add_header_button"),
                                                )
                                                .clicked()
                                                {
                                                    self.header
                                                        .push(("".to_string(), "".to_string()));
                                                };
                                            })
                                        });
                                    });
                            })
                            .response
                            .rect;

                        ui.painter_at(group_rect).rect_stroke(
                            group_rect,
                            Rounding::none(),
                            Stroke::new(2.0, color::GRAY),
                        );
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

                    ScrollArea::vertical()
                        .max_height(ui.available_height())
                        .show(ui, |ui| {
                            ui.group(|ui| {
                                ui.set_width(ui.available_width());
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

    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Http {
    pub fn from_curl(
        url: String,
        method: String,
        header: Vec<(String, String)>,
        text_param: String,
        form_param: Vec<(String, String, Option<PathBuf>, FormParamType)>,
        param_type: String,
    ) -> Result<Self, String> {
        // TODO refactor
        let method = match method.as_str() {
            "GET" => Method::Get,
            "DELETE" => Method::Delete,
            "PUT" => Method::Put,
            "POST" => Method::Post,
            "PATCH" => Method::Patch,
            _ => {
                return Err("No such method".to_string());
            }
        };

        let param_type = if param_type.contains("application/json") {
            ParamType::Json
        } else if param_type.contains("application/json") {
            ParamType::Json
        } else {
            ParamType::Other
        };

        Ok(Self {
            id: get_uuid(),
            name: "New http request".to_string(),
            url,
            method,
            header,
            text_param,
            form_param,
            param_type,
            ..Default::default()
        })
    }

    fn param_type_view(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| match self.method {
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
        });
    }

    fn param_view(&mut self, ui: &mut Ui) {
        // TODO open an issue to trace this style problem:Group's bottom line is lighter than others when form data instead of raw/json.
        let group_rect = ui
            .group(|ui| {
                ui.set_width(ui.available_width());
                ui.vertical_centered(|ui| {
                    ui.add_space(5.0);
                    self.param_type_view(ui);
                    ui.add_space(5.0);
                    ui.style_mut().visuals.widgets = Visuals::light().widgets;
                    ScrollArea::vertical()
                        .max_height(ui.available_height() / 2.0 - get_row_height(ui).1)
                        .show(ui, |ui| {
                            ui.style_mut().visuals.widgets = crate::style::get_widgets(1.0);
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
                                _ => {
                                    ui.set_width(ui.available_width());
                                    // Make panel has max width
                                    // ui.horizontal(|ui| ui.vertical_centered(|ui| ui.add_space(1.0)));
                                }
                            }
                        });
                });
            })
            .response
            .rect;
        ui.painter_at(group_rect).rect_stroke(
            group_rect,
            Rounding::none(),
            Stroke::new(2.0, color::GRAY),
        );
    }

    fn form_data_param_view(&mut self, ui: &mut Ui) {
        ui.set_width(ui.available_width());
        let (font_id, row_height) = crate::style::get_row_height(ui);
        let mut label = 0;
        let col_width = (ui.available_width() - 120.0 - row_height * 3.0) / 2.0;
        self.form_param
            .retain_mut(|(key, value, path_buf, form_param_type)| {
                ui.add_space(2.0);
                !ui.with_layout(Layout::left_to_right().with_cross_align(Align::Min), |ui| {
                    ui.add(TextEdit::singleline(key).desired_width(col_width));

                    label += 1;
                    // TODO center
                    let mut job = LayoutJob::simple(
                        format!("{:?}", form_param_type),
                        font_id.clone(),
                        color::BLACK,
                        70.0,
                    );
                    job.first_row_min_height = row_height + 2.0;
                    ComboBox::from_id_source(label.to_string() + "form_param_type_combo_box")
                        .selected_text(WidgetText::LayoutJob(job))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(form_param_type, FormParamType::Text, "Text");
                            ui.selectable_value(form_param_type, FormParamType::File, "File");
                        });

                    match form_param_type {
                        FormParamType::File => {
                            let file_button = components::widget_with_size(
                                ui,
                                Vec2::new(col_width, row_height + 4.0),
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
                                Vec2::new(col_width, row_height),
                                TextEdit::singleline(value),
                            );
                        }
                    }

                    let mut next_pos = ui.next_widget_position();
                    next_pos = Pos2 {
                        x: next_pos.x,
                        y: next_pos.y + 2.0 + (row_height - row_height / 1.5) / 2.0,
                    };
                    ui.style_mut().visuals.widgets.hovered.expansion = 2.0;
                    let clear_btn_rect =
                        Rect::from_min_max(next_pos, next_pos.add(Vec2::splat(row_height / 1.5)));
                    ui.add_space(row_height);
                    components::close_button(
                        ui,
                        clear_btn_rect,
                        Id::new(label.to_string() + "remove_param_btn"),
                    )
                })
                .inner
                .clicked()
            });
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.vertical_centered(|ui| {
                ui.style_mut().visuals.widgets.hovered.expansion = 2.0;
                let next_pos = ui.next_widget_position();
                let clear_btn_rect =
                    Rect::from_min_max(next_pos, next_pos.add(Vec2::splat(row_height / 1.5)));
                if components::add_button(ui, clear_btn_rect, Id::new("add_form_data_param_button"))
                    .clicked()
                {
                    self.form_param.push((
                        "".to_string(),
                        "".to_string(),
                        None,
                        FormParamType::Text,
                    ));
                };
            })
        });
    }

    fn get_next_del_btn(row_height: f32, ui: &mut Ui) -> Rect {
        let mut next_pos = ui.next_widget_position();
        next_pos = Pos2 {
            x: next_pos.x,
            y: next_pos.y - row_height / 3.0,
        };
        ui.style_mut().visuals.widgets.hovered.expansion = 2.0;
        let clear_btn_rect =
            Rect::from_min_max(next_pos, next_pos.add(Vec2::splat(row_height / 1.5)));
        clear_btn_rect
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

        let (_, row_height) = style::get_row_height(ui);

        let mut label = 1;
        self.form_param.retain_mut(|(key, value, ..)| {
            label += 1;
            ui.add_space(2.0);
            !ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(key).desired_width(col_width));
                ui.add(TextEdit::singleline(value).desired_width(col_width));
                let clear_btn_rect = Self::get_next_del_btn(row_height, ui);
                ui.add_space(row_height);
                components::close_button(
                    ui,
                    clear_btn_rect,
                    Id::new(label.to_string() + "remove_query_param_btn"),
                )
            })
            .inner
            .clicked()
        });
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.vertical_centered(|ui| {
                ui.style_mut().visuals.widgets.hovered.expansion = 2.0;
                let next_pos = ui.next_widget_position();
                let clear_btn_rect =
                    Rect::from_min_max(next_pos, next_pos.add(Vec2::splat(row_height / 1.5)));
                if components::add_button(
                    ui,
                    clear_btn_rect,
                    Id::new("add_query_data_param_button"),
                )
                .clicked()
                {
                    self.form_param.push((
                        "".to_string(),
                        "".to_string(),
                        None,
                        FormParamType::Text,
                    ));
                };
            })
        });
    }

    fn method_select(&mut self, ui: &mut Ui) {
        // ui.spacing_mut().combo_height = 20.0 + 16.0;
        // ui.style_mut().spacing.window_margin = Margin {
        //     left: 5.0,
        //     right: 5.0,
        //     top: 5.0,
        //     bottom: 5.0,
        // };
        // ui.style_mut().visuals.widgets = crate::style::get_widgets();
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

    fn send_button(&mut self, ui: &mut Ui, id: FontId, row_height: f32) {
        match &self.state {
            None => {
                // TODO width of button https://github.com/emilk/egui/blob/master/egui_demo_lib/src/demo/tests.rs
                let mut job = LayoutJob::simple("Send".to_string(), id, color::WHITE, 80.0);
                job.first_row_min_height = row_height + 2.0;

                let send_button = components::widget_with_size(
                    ui,
                    Vec2::new(80.0, row_height + 4.0),
                    Button::new(WidgetText::from("Send").color(color::WHITE))
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
                        Err(err) => self.state = Some(Promise::from_ready(Err(err.to_string()))),
                    };
                };
            }
            Some(promise) => {
                // Cancel the request

                match promise.ready() {
                    None => {
                        if components::widget_with_size(
                            ui,
                            Vec2::new(80.0, row_height + 4.0),
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

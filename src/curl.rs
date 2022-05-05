use std::path::PathBuf;

use eframe::egui::{Button, Context, ScrollArea, TextEdit, Window};

use crate::color;
use crate::request::http::{FormParamType, Http};

#[derive(Default)]
pub struct Curl {
    pub show_curl_window: bool,
    // Fix `Multiple Mutable References in Closures`
    // error[E0500]: closure requires unique access to `self.show_curl_window` but it is already borrowed
    pub temp_show: bool,
    error: Option<String>,
    text: String,
}

impl Curl {
    pub fn draw_curl_window(&mut self, ctx: &Context, callback: impl FnOnce(Http)) {
        self.show_curl_window = self.show_curl_window && self.temp_show;
        Window::new("Import from cURL")
            .resizable(true)
            .open(&mut self.show_curl_window)
            .collapsible(false)
            .show(ctx, |ui| {
                // ui.style_mut().visuals.selection.stroke.color = color::CRIMSON;
                ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    ui.add(TextEdit::multiline(&mut self.text).desired_width(f32::INFINITY))
                });
                if let Some(err) = &self.error {
                    ui.add(
                        TextEdit::singleline(&mut err.as_str())
                            .text_color(color::CRIMSON)
                            .desired_rows(1)
                            .desired_width(f32::INFINITY),
                    );
                }
                ui.horizontal(|ui| {
                    if ui.add(Button::new("Import")).clicked() {
                        {
                            match parse_curl(self.text.as_str()) {
                                Ok(http) => {
                                    self.text = "".to_string();
                                    self.error = None;
                                    self.temp_show = false;
                                    callback(http)
                                }
                                Err(error) => {
                                    self.error = Some(error);
                                }
                            }
                        }
                    }
                })
            });
    }
}

#[test]
fn test() {
    // let str = r#"curl -X POST --location "http://localhost/upload"
    // \ -H "Content-Type: multipart/form-data; boundary=WebAppBoundary"
    // \ -F "fileName=test.docx;type=text/plain"
    // \ -F "file=@test.file;filename=test.file;type=*/*""#;

    let str = r#"curl 'http://localhost/login' \
  -X POST
  -H 'Accept: application/json, text/plain, */*' \
  -H 'Accept-Language: zh-CN,zh;q=0.9,en;q=0.8' \
  -H 'Connection: keep-alive' \
  -H 'Content-Type: application/json;charset=UTF-8' \
  -H 'Cookie: username=admin; rememberMe=true; sidebarStatus=1;' \
  -H 'Origin: http://localhost' \
  -H 'Referer: http://localhost/login' \
  -H 'Token: ' \
  -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36' \
  --data-raw '{"username":"admin","password":"admin"}' \
  --compressed \
  --insecure"#;
    assert!(parse_curl(str).is_ok());
}

fn parse_curl(curl: &str) -> Result<Http, String> {
    if curl.is_empty() {
        return Err("Empty curl command".to_string());
    }

    let result = match shellwords::split(curl) {
        Ok(vec) => vec,
        Err(err) => return Err(err.to_string()),
    };

    let mut header_vec = vec![];

    let mut url = None;
    let mut text_param = "".to_string();
    let mut form_param = vec![];
    let mut method = String::from("GET");
    let mut param_type = "".to_string();

    let mut i = 0;
    while i < result.len() {
        if let Some(value) = result.get(i) {
            match value.as_str() {
                "-H" | "--header" => {
                    i = i + 1;
                    if let Some(header) = result.get(i) {
                        let result = split_string(":", header.as_str());
                        if header.to_lowercase().contains("content-type") {
                            param_type = result.1.clone();
                        }

                        header_vec.push(result);
                    }
                }
                "-F" | "--form" => {
                    i = i + 1;
                    if let Some(param) = result.get(i) {
                        let string = split_string("=", &split_string(";", param).0);
                        let param = if string.1.starts_with("@") {
                            (
                                string.0,
                                "".to_string(),
                                Some(PathBuf::from(string.1.replace("@", ""))),
                                FormParamType::File,
                            )
                        } else {
                            (string.0, string.1, None, FormParamType::Text)
                        };
                        form_param.push(param);
                    }
                }
                "--data-binary" => {}
                "--data" | "--data-raw" | "-d" => {
                    i = i + 1;
                    if let Some(param) = result.get(i) {
                        text_param = param.clone()
                    }
                }
                "-X" | "--request" => {
                    i = i + 1;
                    if let Some(method_result) = result.get(i) {
                        method = method_result.to_uppercase()
                    }
                }
                _ => {
                    if !value.starts_with("-") {
                        url = Some(value.to_string());
                    }
                }
            };
            i = i + 1
        };
    }

    if url.is_none() {
        return Err("No url found".to_string());
    }

    Http::from_curl(
        url.unwrap(),
        method,
        header_vec,
        text_param,
        form_param,
        param_type,
    )
}

fn split_string(regex: &str, origin: &str) -> (String, String) {
    return match origin.find(regex) {
        None => (origin.to_string(), "".to_string()),
        Some(index) => (
            origin[0..index].to_string(),
            origin[(index + 1)..origin.len()].to_string(),
        ),
    };
}

#[test]
fn test_split_string() {
    let regex = ":";
    let first_str = ":test";
    let last_str = "test:";
    let middle_str = "test:test";
    let multiple_regex = "test:test::";

    assert_eq!(
        split_string(regex, first_str),
        ("".to_string(), "test".to_string())
    );
    assert_eq!(
        split_string(regex, last_str),
        ("test".to_string(), "".to_string())
    );
    assert_eq!(
        split_string(regex, middle_str),
        ("test".to_string(), "test".to_string())
    );
    assert_eq!(
        split_string(regex, multiple_regex),
        ("test".to_string(), "test::".to_string())
    )
}

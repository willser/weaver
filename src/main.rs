#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::text_input::State;
use iced::{
    button, executor, pick_list, scrollable, slider, text_input, Align, Application, Button,
    Clipboard, Column, Command, Container, Element, HorizontalAlignment, Length, PickList, Radio,
    Row, Rule, Scrollable, Settings, Text, TextInput,
};

use std::fmt::{Alignment, Display, Formatter};

fn main() {
    Weaver::run(Settings {
        antialiasing: true,
        // TODO custom font
        // default_font: Some(include_bytes!("C:\\Windows\\Fonts\\simsun.ttc")),
        ..Settings::default()
    });
}

#[derive(Debug, Default)]
struct Weaver {
    url_input: text_input::State,
    url: String,
    param_json_input: text_input::State,
    param_json: String,
    send_button: button::State,
    header: Vec<RequestHeader>,
    body: RequestBody,
    content_type: ContentType,
    method: Method,
    method_pick_list: pick_list::State<Method>,
    method_scroll: scrollable::State,
    request_info: RequestInfo,
    request_info_is_param: bool,
    add_header_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    SendButtonPressed,
    UrlChanged(String),
    ParamJsonChange(String),
    ContentTypeChanged(ContentType),
    MethodSelected(Method),
    RequestInfoChanged(bool),
    ChangeHeaderKey(usize, String),
    ChangeHeaderValue(usize, String),
    AddHeader,
    DelHeader(usize),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Method {
    // GET,
    POST,
}

impl Default for Method {
    fn default() -> Self {
        Self::POST
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Method::GET => "GET",
                Method::POST => "POST",
            }
        )
    }
}

#[derive(Debug, Default)]
struct RequestHeader {
    key: String,
    value: String,
    key_input: text_input::State,
    value_input: text_input::State,
    delete_button: button::State,
}

#[derive(Debug, Default)]
struct RequestBody {
    form_data: Vec<(String, String)>,
    raw: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContentType {
    FormData,
    Json,
}

impl Default for ContentType {
    fn default() -> Self {
        ContentType::FormData
    }
}

impl ContentType {
    pub const ALL: [ContentType; 2] = [ContentType::FormData, ContentType::Json];
}

#[derive(Debug, Default, Clone)]
pub struct RequestInfo {
    header: button::State,
    param: button::State,
}

impl RequestInfo {
    fn view(&mut self, is_param: bool) -> Element<Message> {
        let RequestInfo { header, param } = self;

        let get_button = |state, label, _now_is_param: bool, is_param| {
            let label = Text::new(label).size(16);
            let button = Button::new(state, label);

            // todo different style
            button
                .on_press(Message::RequestInfoChanged(is_param))
                .padding(8)
        };

        Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .spacing(50)
                    .align_items(Align::Center)
                    .push(get_button(header, "REQUEST HEADER", is_param, false))
                    .push(get_button(param, "PARAM", is_param, true)),
            )
            .into()
    }
}

impl Application for Weaver {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { ..Self::default() }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Weaver")
    }

    fn update(
        &mut self,
        message: Self::Message,
        clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::SendButtonPressed => Command::none(),
            Message::UrlChanged(value) => {
                self.url = value;
                Command::none()
            }
            Message::ContentTypeChanged(content_type) => {
                self.content_type = content_type;
                Command::none()
            }
            Message::MethodSelected(method) => {
                self.method = method;
                Command::none()
            }
            Message::RequestInfoChanged(info) => {
                self.request_info_is_param = info;
                Command::none()
            }

            Message::ChangeHeaderKey(index, value) => {
                self.header[index].key = value;
                Command::none()
            }
            Message::ChangeHeaderValue(index, value) => {
                self.header[index].value = value;
                Command::none()
            }
            Message::AddHeader => {
                self.header.push(RequestHeader::default());
                Command::none()
            }
            Message::DelHeader(index) => {
                self.header.remove(index);
                Command::none()
            }
            Message::ParamJsonChange(value) => {
                self.param_json = value;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let button = Button::new(&mut self.send_button, Text::new("Send"))
            .padding(10)
            .on_press(Message::SendButtonPressed);

        let text_input = TextInput::new(
            &mut self.url_input,
            "URL...",
            &self.url,
            Message::UrlChanged,
        )
        .padding(10)
        .size(20);

        let pick_list = PickList::new(
            &mut self.method_pick_list,
            vec![Method::POST],
            Some(self.method),
            Message::MethodSelected,
        );
        // .style(style::Theme::Light);

        let mut method_list = Scrollable::new(&mut self.method_scroll)
            .align_items(Align::Center)
            .spacing(10)
            .push(pick_list);
        // .style(style::Theme::Light);

        let request_info: Element<Message> = match self.request_info_is_param {
            false => {
                self.header
                    .iter_mut()
                    .enumerate()
                    .fold(
                        Column::new().align_items(Align::Center).width(Length::Fill),
                        |column, (index, header)| {
                            column.push(
                                Row::new()
                                    .push(TextInput::new(
                                        &mut header.key_input,
                                        "",
                                        &header.key,
                                        move |message| Message::ChangeHeaderKey(index, message),
                                    ))
                                    .push(TextInput::new(
                                        &mut header.value_input,
                                        "",
                                        &header.value,
                                        move |message| Message::ChangeHeaderValue(index, message),
                                    ))
                                    .push(
                                        Button::new(&mut header.delete_button, Text::new("DEL"))
                                            .padding(10)
                                            .on_press(Message::DelHeader(index)),
                                    ),
                            )
                        },
                    )
                    .push(
                        Row::new().align_items(Align::Center).push(
                            Button::new(&mut self.add_header_button, Text::new("+"))
                                .padding(10)
                                .on_press(Message::AddHeader),
                        ),
                    )
                    .into()
                // Row::new()
                //     .spacing(10)
                //     .push(Text::new("Request Header:"))
                //     .align_items(Align::Center)
                //     .into()
            }
            true => Column::new()
                .width(Length::Fill)
                .align_items(Align::Center)
                .push(
                    Row::new()
                        .spacing(10)
                        .push(Text::new("Content type:"))
                        .push(Radio::new(
                            ContentType::FormData,
                            format!("x-www-form-urlencoded;charset=UTF-8"),
                            Some(self.content_type),
                            Message::ContentTypeChanged,
                        ))
                        .push(Radio::new(
                            ContentType::Json,
                            format!("json"),
                            Some(self.content_type),
                            Message::ContentTypeChanged,
                        )),
                )
                .push(match self.content_type {
                    ContentType::FormData => Column::new().push(Text::new("formdata")),
                    ContentType::Json => Column::new().push(Row::new().push(TextInput::new(
                        &mut self.param_json_input,
                        "",
                        &self.param_json,
                        Message::ParamJsonChange,
                    ))),
                })
                .into(),
        };
        let content = Column::new()
            .spacing(20)
            .padding(20)
            // .push(Row::new().spacing(10).push(content_type))
            .push(
                Row::new()
                    .spacing(10)
                    .push(method_list)
                    .push(text_input)
                    .push(button),
            )
            // .push(Rule::horizontal(38))
            .push(self.request_info.view(self.request_info_is_param))
            .push(Rule::horizontal(5))
            .push(request_info);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .align_y(Align::Start)
            .into()
    }
}

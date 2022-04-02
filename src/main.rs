#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::text_input::State;
use iced::{
    button, executor, pick_list, scrollable, slider, text_input, Align, Application, Button,
    Clipboard, Column, Command, Container, Element, Length, PickList, Radio, Row, Rule, Scrollable,
    Settings, Text, TextInput,
};

use std::borrow::BorrowMut;
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
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    button: button::State,
    header: Vec<(String, String, text_input::State, text_input::State)>,
    body: RequestBody,
    content_type: ContentType,
    method: Method,
    method_pick_list: pick_list::State<Method>,
    request_info: RequestInfo,
    request_info_is_param: bool,
}

#[derive(Debug, Clone)]
enum Message {
    SendButtonPressed,
    UrlChanged(String),
    ContentTypeChanged(ContentType),
    MethodSelected(Method),
    RequestInfoChanged(bool),
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
    fn view(&mut self, is_param: bool) -> Row<Message> {
        let RequestInfo { header, param } = self;

        let get_button = |state, label, _now_is_param: bool, is_param| {
            let label = Text::new(label).size(16);
            let button = Button::new(state, label);

            // todo different style
            button
                .on_press(Message::RequestInfoChanged(is_param))
                .padding(8)
        };

        Row::new().spacing(20).align_items(Align::Center).push(
            Row::new()
                .width(Length::Shrink)
                .spacing(10)
                .push(get_button(header, "REQUEST HEADER", is_param, false))
                .push(get_button(param, "PARAM", is_param, true)),
        )
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
                self.input_value = value;
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
        }
    }

    fn view(&mut self) -> Element<Message> {
        let button = Button::new(&mut self.button, Text::new("Send"))
            .padding(10)
            .on_press(Message::SendButtonPressed);

        let text_input = TextInput::new(
            &mut self.input,
            "URL...",
            &self.input_value,
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

        let mut content = Scrollable::new(&mut self.scroll)
            .align_items(Align::Center)
            .spacing(10)
            .push(pick_list);
        // .style(style::Theme::Light);

        let element: Element<Message> = match self.request_info_is_param {
            false => Row::new()
                .spacing(10)
                .push(Text::new("Request Header:"))
                .align_items(Align::Center)
                .into(),
            true => Row::new()
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
                ))
                .align_items(Align::Center)
                .into(),
        };
        let content = Column::new()
            .spacing(20)
            .padding(20)
            // .push(Row::new().spacing(10).push(content_type))
            .push(
                Row::new()
                    .spacing(10)
                    .push(content)
                    .push(text_input)
                    .push(button),
            )
            // .push(Rule::horizontal(38))
            .push(self.request_info.view(self.request_info_is_param))
            .push(Rule::horizontal(5))
            .push(element);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .align_y(Align::Start)
            .into()
    }
}

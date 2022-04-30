use iced::{Text, Settings, text_input, Sandbox, Element, Column};

mod lib;
use lib::parselatin;

#[derive(Default, Debug, Clone)]
struct CyrillicKeyboard {
    text: String,
    input: text_input::State
}


#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
}

impl Sandbox for CyrillicKeyboard {

    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("AguMatrix")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.text = parselatin(value)
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        
        Column::new()
        .padding(20)
        .spacing(20)
        .align_items(iced::Align::Center)
        .push(
            iced::Row::new()
            .padding(20)
            .height(iced::Length::Shrink)
            .align_items(iced::Align::Center)
            .push(
                Text::new("Cyrillic Keyboard")
                .size(60)
            )
        )
        .push(
            iced::Row::new()
            .padding(20)
            .height(iced::Length::Shrink)
            .align_items(iced::Align::Center)
            .push(
                Text::new(self.text.to_string())
                .size(24)
            )
        )
        .push(
            iced::Row::new()
            .padding(20)
            .height(iced::Length::Shrink)
            .align_items(iced::Align::Center)
            .push(
                text_input::TextInput::new(&mut self.input, "Write in Latin alphabet here...", &self.text, Message::InputChanged)
                .padding(10)
            )

        )
        .height(iced::Length::Shrink)
        .into()
        
    }
}

fn main() -> iced::Result {
    CyrillicKeyboard::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
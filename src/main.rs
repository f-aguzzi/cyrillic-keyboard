use iced::{ Text, Settings, text_input, Sandbox, Element, Column };

mod lib;
use lib::{ parselatin, clipboard };

#[derive(Default, Debug, Clone)]
struct CyrillicKeyboard {
    text: String,
    convertedtext: String,
    input: text_input::State,
    button: iced::button::State
}


#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ButtonClicked,
}

impl Sandbox for CyrillicKeyboard {

    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Cyrillic Keyboard")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.convertedtext = parselatin(&value);
                self.text = value
            },
            Message::ButtonClicked => {
                clipboard(&self.convertedtext);
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
                iced::Text::new("Cyrillic Keyboard")
                .size(60)
            )
        )
        .push(
            iced::Row::new()
            .padding(20)
            .height(iced::Length::Shrink)
            .align_items(iced::Align::Center)
            .push(
                Text::new(self.convertedtext.to_string())
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
        .push(
            iced::Button::new(&mut self.button, Text::new("Copy to clipboard"))
            .on_press(Message::ButtonClicked)
            .padding(12)
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
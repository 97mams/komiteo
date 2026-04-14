use iced::{Element, widget::{text}};

#[derive(Debug, Default)]
struct AppState {
    // Define your application state here
}

#[derive(Debug)]
enum Message {
    Exit,
}

fn update(state:&mut AppState, message: Message) {
    match message {
        Message::Exit => {
        }
    }
}

fn view(state: &AppState) -> Element<Message> {
    text("Hello, world!").into()
}

fn main() ->iced::Result {
   iced::run(update, view)
}
use core::panic;

pub fn main() -> iced::Result {
    iced::run("Chattercat live audio visualizer", update, view)
}

#[derive(Debug, Clone)]
enum Message {
    CheckboxToggled(bool),
}

#[derive(Default, Debug)]
struct State {
    checkbox_state: bool,
}

fn update(state: &mut State, msg: Message) {
    match msg {
        Message::CheckboxToggled(ck_state) => {
            state.checkbox_state = ck_state;
        }
        _ => panic!("Got unexpected type of Message: {:#?}", msg),
    }
}

fn view(state: &State) -> iced::Element<Message> {
    iced::widget::checkbox("This is a stub checkbox", state.checkbox_state)
        .on_toggle(Message::CheckboxToggled)
        .into()
}

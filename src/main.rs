use iced::{image, widget::button, Alignment, Button, Element, Row, Sandbox, Settings, Text};

fn main() -> iced::Result {
    GifProject::run(Settings::default())
}

struct GifProject {
    // A vector of GIF image frames
    frames: Vec<image::Handle>,

    // The index of the frame being operated on
    current: Option<u32>,

    step_forward_button: button::State,
    step_backward_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    FrameStepForward,
    FrameStepBackward,
}

impl Sandbox for GifProject {
    type Message = Message;

    fn new() -> Self {
        Self {
            frames: Vec::new(),
            current: None,
            step_forward_button: button::State::default(),
            step_backward_button: button::State::default(),
        }
    }

    fn title(&self) -> String {
        String::from("gifhacker")
    }

    fn view(&mut self) -> Element<Message> {
        Row::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.step_backward_button, Text::new("<<"))
                    .on_press(Message::FrameStepBackward),
            )
            .push(Text::new("ok").size(50))
            .push(
                Button::new(&mut self.step_forward_button, Text::new(">>"))
                    .on_press(Message::FrameStepForward),
            )
            .into()
    }

    fn update(&mut self, message: Message) {
        match (message, self.current) {
            (Message::FrameStepForward, Some(x)) => self.current = Some(x + 1),
            (Message::FrameStepBackward, Some(x)) => self.current = Some(x - 1),
            _ => (),
        }
    }
}

use iced::{
    image, widget::button, Alignment, Application, Button, Column, Command, Element, Row, Settings,
    Text,
};

fn main() -> iced::Result {
    GifProject::run(Settings::default())
}

#[derive(Debug, Clone)]
enum GifProject {
    NoProject,
    Editing {
        // A vector of GIF image frames
        frames: Vec<image::Handle>,
        frame_viewer: image::viewer::State,
        // The index of the frame being operated on
        current: i32,
        step_forward_button: button::State,
        step_backward_button: button::State,
    },
    Errored,
}

#[derive(Debug, Clone)]
enum Message {
    FrameStepForward,
    FrameStepBackward,
    ProjectLoaded(Result<GifProject, Error>),
}

#[derive(Debug, Clone)]
enum Error {
    RequestError,
    DecodingError,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::RequestError
    }
}

impl From<gif::DecodingError> for Error {
    fn from(error: gif::DecodingError) -> Error {
        dbg!(error);

        Error::DecodingError
    }
}

impl GifProject {
    async fn init() -> Result<Self, Error> {
        let project_file = Self::fetch_image(
            "https://cdn.discordapp.com/emojis/643066491494727680.gif?size=128&quality=lossless",
        )
        .await?;

        Ok(Self::Editing {
            frames: project_file,
            frame_viewer: image::viewer::State::new(),
            current: -1,
            step_forward_button: button::State::default(),
            step_backward_button: button::State::default(),
        })
    }

    async fn fetch_image(url: &str) -> Result<Vec<image::Handle>, Error> {
        let bytes = reqwest::get(url).await?.bytes().await?;
        let bytes_vec = bytes.to_vec();
        let reader = std::io::Cursor::new(bytes_vec);

        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);

        let mut decoder = options.read_info(reader)?;

        let mut frames: Vec<image::Handle> = Vec::new();
        while let Ok(Some(frame)) = decoder.read_next_frame() {
            frames.push(image::Handle::from_memory(frame.buffer.to_vec()))
        }

        Ok(frames)
    }
}

impl Application for GifProject {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::NoProject,
            Command::perform(Self::init(), Message::ProjectLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("gifhacker")
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Self::Editing {
                frames,
                frame_viewer,
                current,
                step_backward_button,
                step_forward_button,
                ..
            } => Column::new()
                .padding(20)
                .push(Row::new().push(image::Viewer::new(frame_viewer, frames[0].clone())))
                .push(
                    Row::new()
                        .padding(20)
                        .align_items(Alignment::Center)
                        .push(
                            Button::new(step_backward_button, Text::new("<<"))
                                .on_press(Message::FrameStepBackward),
                        )
                        .push(Text::new(current.to_string()).size(50))
                        .push(
                            Button::new(step_forward_button, Text::new(">>"))
                                .on_press(Message::FrameStepForward),
                        ),
                )
                .into(),
            Self::Errored => Column::new()
                .padding(20)
                .push(Text::new("An error has occurred."))
                .into(),
            _ => Row::new().into(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FrameStepForward => match self {
                Self::Editing {
                    current,
                    frames,
                    frame_viewer,
                    step_forward_button,
                    step_backward_button,
                } => {
                    *self = Self::Editing {
                        current: *current + 1,
                        frames: frames.to_vec(),
                        frame_viewer: *frame_viewer,
                        step_forward_button: *step_forward_button,
                        step_backward_button: *step_backward_button,
                    };

                    Command::none()
                }
                _ => Command::none(),
            },
            Message::FrameStepBackward => match self {
                Self::Editing {
                    current,
                    frames,
                    frame_viewer,
                    step_forward_button,
                    step_backward_button,
                } => {
                    *self = Self::Editing {
                        current: *current - 1,
                        frames: frames.to_vec(),
                        frame_viewer: *frame_viewer,
                        step_forward_button: *step_forward_button,
                        step_backward_button: *step_backward_button,
                    };

                    Command::none()
                }
                _ => Command::none(),
            },
            Message::ProjectLoaded(Ok(project)) => {
                *self = project;
                Command::none()
            }
            Message::ProjectLoaded(Err(_error)) => {
                *self = Self::Errored;
                Command::none()
            }
        }
    }
}

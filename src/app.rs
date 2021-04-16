use crate::analyzer;
use iced::{
    executor, Align, Application, Clipboard, Column, Command, Element, Rule, Subscription, Text,
};
use iced_native::keyboard::{Event::KeyPressed, KeyCode};

#[derive(Default)]
pub struct Pres {
    slide_num: usize,
    dark_bg: bool,
    exit: bool,
    slides: Box<analyzer::presentation::PresYml>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NextSlide,
    PrevSlide,
    ExitPres,
    ChangeTheme,
}

#[derive(Debug)]
pub struct Flags {
    pub pres: Box<analyzer::presentation::PresYml>,
}

impl Application for Pres {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(flags: Flags) -> (Pres, Command<Message>) {
        (
            Pres {
                dark_bg: false,
                slides: flags.pres,
                ..Pres::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Slide number {}", self.slide_num)
    }

    fn update(&mut self, msg: Message, _cb: &mut Clipboard) -> Command<Message> {
        match msg {
            Message::ChangeTheme => self.dark_bg = !self.dark_bg,
            Message::NextSlide => {
                if self.slide_num < self.slides.pres.slides.len() - 1 {
                    self.slide_num += 1
                }
            }
            Message::PrevSlide => {
                if self.slide_num > 0 {
                    self.slide_num -= 1
                }
            }
            Message::ExitPres => self.exit = true,
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events_with(|e, _s| match e {
            iced_native::Event::Keyboard(KeyPressed { key_code: kk, .. }) => match kk {
                KeyCode::Left => Some(Message::PrevSlide),
                KeyCode::Right => Some(Message::NextSlide),
                KeyCode::Escape => Some(Message::ExitPres),
                KeyCode::P => Some(Message::ChangeTheme),
                _ => None,
            },
            _ => None,
        })
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(format!("Slide - {}", self.slide_num)).size(50))
            .push(Rule::horizontal(5))
            .push(
                Text::new(format!(
                    "{:?}",
                    self.slides.pres.slides.get(self.slide_num).unwrap()
                ))
                .size(50),
            )
            .into()
    }

    fn background_color(&self) -> iced::Color {
        if self.dark_bg {
            iced::Color::BLACK
        } else {
            iced::Color::WHITE
        }
    }

    fn should_exit(&self) -> bool {
        self.exit
    }
}

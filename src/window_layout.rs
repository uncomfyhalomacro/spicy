use iced::window;
use iced::{Container, Element, Image, Length, Sandbox, Settings, Text};
use log::{self, error};
use std::env;
use std::path::Path;
// use std::path::Path;
use std::{fs, result::Result};

pub fn present() {
    match ImageWindow::run(Settings {
        antialiasing: false,
        window: window::Settings {
            size: (1024, 768),
            min_size: Some((256, 192)),
            ..window::Settings::default()
        },
        ..Settings::default()
    }) {
        Result::Ok(val) => val,
        Result::Err(err) => error!("Window failed to load -> {}", err),
    }
}

struct ImageWindow;

impl Sandbox for ImageWindow {
    type Message = ();

    fn new() -> Self {
        ImageWindow
    }
    fn title(&self) -> String {
        String::from("Spicy Image Window")
    }

    fn update(&mut self, _message: ()) {}

    fn view(&mut self) -> Element<()> {
        // going to use clap or something here to point a path.
        // for now this is just a place holder.
        let arg = if env::args().count() == 2 {
            env::args().nth(1).unwrap()
        } else {
            panic!("Please enter a target file path")
        };

        let img_path = Path::new(&arg);


        match fs::metadata(&img_path) {
            Result::Ok(img_file) => {
                if img_file.is_file() {
                    let img = Image::new(img_path)
                        .height(Length::Fill)
                        .width(Length::Fill);
                    Container::new(img)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x()
                        .center_y()
                        .into()
                } else if img_file.is_dir() {
                    error!("Path is not a file.");
                    Container::new(Text::new("Path is not a file!").size(48))
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .center_x()
                        .center_y()
                        .into()
                } else {
                    panic!("Unknown error!");
                }
            }
            Result::Err(err) => Container::new(Text::new(format!("Error: {}", err)).size(48))
                .height(Length::Fill)
                .width(Length::Fill)
                .center_x()
                .center_y()
                .into(),
        }
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::BLACK
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn run(settings: Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }
}

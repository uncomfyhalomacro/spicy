use iced::Sandbox;
use iced::Settings;
use iced::{Container, Element, Image, Length, Text};
use log::{self, error};
use std::{fs, result::Result};

pub fn present() {
    match ImageWindow::run(Settings::default()) {
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
        let img_path = format!(
            "{}/resources/2022012615271056218.png",
            env!("CARGO_MANIFEST_DIR")
        );
        match fs::metadata(&img_path) {
            Result::Ok(img_file) if img_file.is_file() => {
                let img = Image::new(img_path)
                    .height(Length::Fill)
                    .width(Length::Fill);
                Container::new(img)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            Result::Ok(img_file) if img_file.is_dir() => {
                error!("Path is not a file.");
                Container::new(Text::new("Path is not a file!").size(48))
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            Result::Err(err) => {
                Container::new(Text::new(format!("Error: {}", err)).size(48))
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            Result::Ok(_) => {
                Container::new(Text::new("Unknown error").size(48))
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
        }
    }
}

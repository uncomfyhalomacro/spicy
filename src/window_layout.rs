use iced::window;
use iced::{Container, Element, Image, Length, Sandbox, Settings, Text};
use log::{self, error};
use std::{fs, result::Result};

pub fn present() {
    match ImageWindow::run(spicy_settings()) {
        Result::Ok(val) => val,
        Result::Err(err) => error!("Window failed to load -> {}", err),
    }
}

fn spicy_settings() -> iced::Settings<()> {
    Settings {
        window: window::Settings {
            size: (1024, 768),
            min_size: Some((256, 192)),
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
        },

        flags: Default::default(),
        antialiasing: false,
        default_text_size: 20,
        default_font: Default::default(),
        exit_on_close_request: true,
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
            Result::Err(err) => Container::new(Text::new(format!("Error: {}", err)).size(48))
                .height(Length::Fill)
                .width(Length::Fill)
                .center_x()
                .center_y()
                .into(),
            Result::Ok(_) => Container::new(Text::new("Unknown error").size(48))
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

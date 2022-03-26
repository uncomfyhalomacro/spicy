// use gtk::prelude::*;
// use gtk::Application;
// use gtk::ApplicationWindow;
use iced::{ Container, Image, Length, Element};
use iced::Settings;
use iced::Sandbox;

// pub fn basic_window() {
//     let app = Application::builder()
//     .application_id("org.gtk-rs.example")
//         .build();
//
//     // Connect to "activate" signal of `app`
//     app.connect_activate(build_ui);
//
//     // Run the application
//     app.run();
// }
//
// pub fn build_ui(app: &Application) {
//     let window = ApplicationWindow::builder()
//         .application(app)
//         .title("Spicy Screenshot Editor")
//         .build();
//
//     // Present window
//     window.present();
// }

pub fn present() -> iced::Result {
    ImageWindow::run(Settings::default())
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

    fn update(&mut self, _message: ())   {
        
    }

    fn view(&mut self) -> Element<()> {
        let img = Image::new(format!(
            "{}/resources/2022012615271056218.png",
            env!("CARGO_MANIFEST_DIR")
        ))
        .height(Length::Fill)
        .width(Length::Fill);
        Container::new(img)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

}

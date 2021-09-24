//#![windows_subsystem = "windows"]

mod download;

use iced::window;
use iced::window::Icon;
use iced::{
    Align, button, executor, Application, Button, Clipboard, Column, Command, Container, Element,
    HorizontalAlignment, Length, ProgressBar, Row, Settings, Subscription, Text, VerticalAlignment,
};
use std::collections::HashMap;
use std::env;
use std::io::Cursor;
use std::path::PathBuf;
use std::process::Command as Cmd;

fn main() -> iced::Result {
    let icon_pixels = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 217,
        37, 37, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        226, 38, 35, 152, 224, 37, 37, 253, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 255, 51, 51, 5, 225, 37, 37, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 225, 44, 38, 252, 255, 0, 0, 3, 225, 42, 38,
        60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 223, 37, 37, 253, 234, 38,
        38, 246, 255, 0, 0, 1, 0, 0, 0, 0, 255, 63, 63, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 225,
        43, 39, 252, 232, 47, 39, 233, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 226, 28, 28, 9, 225, 37, 37, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 225, 38, 37, 255, 227, 44, 39, 255, 226, 43, 38, 255, 224, 42,
        38, 254, 241, 45, 41, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 191, 63, 63, 4, 255, 0, 0,
        2, 224, 37, 37, 253, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 3, 225, 37,
        37, 255, 227, 45, 39, 255, 255, 164, 68, 255, 255, 168, 68, 255, 225, 40, 38, 255, 225, 39,
        37, 255, 223, 37, 37, 252, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 223, 38, 37, 253, 226, 28,
        28, 9, 223, 42, 38, 253, 0, 0, 0, 0, 226, 36, 36, 250, 225, 37, 37, 255, 225, 37, 37, 255,
        255, 166, 68, 255, 255, 166, 68, 255, 255, 217, 86, 255, 226, 40, 38, 255, 225, 38, 37,
        255, 225, 37, 37, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 225, 37, 37, 255, 255, 0, 0, 2,
        230, 42, 39, 255, 0, 0, 0, 0, 254, 165, 68, 255, 227, 45, 39, 255, 255, 172, 70, 255, 255,
        169, 69, 255, 255, 169, 69, 255, 255, 235, 93, 255, 255, 170, 68, 255, 225, 38, 37, 255,
        225, 37, 37, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 1, 225, 37, 37, 255, 228, 38, 38, 254,
        226, 43, 38, 255, 255, 167, 68, 255, 255, 203, 81, 255, 255, 177, 73, 255, 255, 166, 68,
        255, 255, 167, 68, 255, 255, 171, 70, 255, 255, 237, 94, 255, 255, 167, 68, 255, 255, 165,
        67, 255, 227, 45, 39, 255, 0, 0, 0, 0, 0, 0, 0, 0, 224, 36, 36, 183, 225, 37, 37, 255, 225,
        37, 37, 255, 255, 164, 67, 255, 255, 166, 68, 255, 253, 235, 93, 255, 255, 237, 98, 255,
        255, 237, 95, 255, 255, 236, 94, 255, 255, 237, 95, 255, 255, 237, 95, 255, 252, 244, 197,
        255, 255, 250, 102, 255, 255, 162, 67, 255, 226, 43, 38, 255, 225, 38, 37, 245, 225, 37,
        37, 255, 225, 38, 37, 255, 225, 37, 37, 255, 255, 163, 67, 255, 255, 167, 68, 255, 248,
        247, 235, 255, 152, 124, 144, 255, 255, 237, 97, 255, 255, 237, 96, 255, 255, 237, 99, 255,
        252, 245, 198, 255, 252, 245, 198, 255, 174, 154, 183, 255, 253, 238, 95, 255, 255, 165,
        68, 255, 255, 164, 67, 255, 226, 40, 38, 255, 223, 38, 37, 254, 225, 38, 37, 255, 255, 172,
        69, 255, 255, 164, 68, 255, 58, 41, 48, 255, 59, 2, 3, 255, 185, 163, 132, 255, 79, 30, 30,
        255, 71, 20, 19, 255, 70, 19, 18, 255, 115, 22, 25, 255, 36, 31, 49, 255, 252, 233, 94,
        255, 255, 165, 68, 255, 249, 136, 60, 255, 225, 38, 37, 255, 0, 0, 0, 0, 225, 38, 37, 255,
        247, 132, 60, 255, 255, 164, 67, 255, 60, 39, 45, 255, 61, 6, 7, 255, 178, 153, 126, 255,
        176, 156, 185, 255, 176, 156, 185, 255, 122, 78, 69, 255, 70, 19, 18, 255, 35, 31, 50, 255,
        255, 248, 98, 255, 255, 164, 67, 255, 225, 38, 37, 255, 225, 38, 37, 255, 225, 37, 37, 255,
        223, 38, 37, 254, 255, 167, 68, 255, 255, 224, 92, 255, 66, 28, 29, 255, 63, 17, 20, 255,
        255, 255, 210, 255, 176, 156, 185, 255, 176, 156, 185, 255, 176, 157, 188, 255, 71, 21, 20,
        255, 34, 32, 51, 255, 202, 173, 73, 255, 237, 86, 49, 255, 225, 37, 37, 255, 225, 37, 37,
        255, 225, 37, 37, 255,
    ];
    let steam_path: PathBuf = ["Steam", "steamapps", "common", "Crowkart"]
        .iter()
        .collect();
    let (bepinex_url, crowkart_game, crowkart_path) = get_os_specific_values(steam_path);
    check_bepinex_install(bepinex_url, crowkart_game, crowkart_path)
        .expect("Could not check BepInEx installation.");
    let icon = Icon::from_rgba(icon_pixels, 16, 16).unwrap();

    App::run(Settings {
        window: window::Settings {
            icon: Option::from(icon),
            size: (320, 640),
            transparent: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

fn get_os_specific_values(steam_path: PathBuf) -> (String, String, PathBuf) {
    let bepinex_url: String;
    let crowkart_game: String;
    let crowkart_path: PathBuf;
    match env::consts::OS {
        "linux" => {
            bepinex_url = String::from("https://github.com/BepInEx/BepInEx/releases/download/v5.4.15/BepInEx_unix_5.4.15.0.zip");
            crowkart_game = String::from("Crowkart");
            crowkart_path = [
                &env::var("user.home").unwrap(),
                ".local",
                ".share",
                steam_path.to_str().unwrap(),
            ]
            .iter()
            .collect();
        }
        "macos" => {
            bepinex_url = String::from("https://github.com/BepInEx/BepInEx/releases/download/v5.4.15/BepInEx_unix_5.4.15.0.zip");
            crowkart_game = String::from("Crowkart.app");
            crowkart_path = [
                &env::var("user.home").unwrap(),
                "Library",
                "Application Support",
                steam_path.to_str().unwrap(),
            ]
            .iter()
            .collect();
        }
        "windows" => {
            bepinex_url = String::from("https://github.com/BepInEx/BepInEx/releases/download/v5.4.15/BepInEx_x64_5.4.15.0.zip");
            crowkart_game = String::from("Crowkart.exe");
            crowkart_path = [r"C:\", "Program Files (x86)", steam_path.to_str().unwrap()]
                .iter()
                .collect();
        }
        _ => panic!("OS not supported."),
    }

    (bepinex_url, crowkart_game, crowkart_path)
}

fn check_bepinex_install(
    bepinex_url: String,
    crowkart_game: String,
    crowkart_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = crowkart_path.as_path();
    if path.join("BepInEx").is_dir() {
        println!("BepInEx already installed");
    } else {
        println!("Downloading BepInEx");
        let response = reqwest::blocking::get(bepinex_url).unwrap();
        let content = response.bytes().unwrap();
        let reader = Cursor::new(content);
        let zip = unzip::Unzipper::new(reader, path);
        zip.unzip().expect("Unable to unzip file");
        Cmd::new(
            [crowkart_path, PathBuf::from(crowkart_game)]
                .iter()
                .collect::<PathBuf>(),
        )
        .output()
        .expect("Failed to launch Crowkart.");
    }

    Ok(())
}

struct App {
    current_download: Download,
    install_buttons: Vec<(String, HashMap<String, String>, button::State)>,
    open_file_explorer_cmd: String,
    plugins_path: PathBuf,
    show_plugins: button::State,
    theme: style::Theme,
}

#[derive(Clone, Debug)]
enum Message {
    Install(String),
    DownloadProgressed((usize, download::Progress)),
    ShowPlugins,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        let rel_plugins_path: PathBuf = [
            "Steam",
            "steamapps",
            "common",
            "Crowkart",
            "BepInEx",
            "plugins",
        ]
        .iter()
        .collect();

        let plugins_path: PathBuf;
        let open_file_explorer_cmd: String;
        match env::consts::OS {
            "linux" => {
                open_file_explorer_cmd = String::from("xdg-open");
                plugins_path = [
                    &env::var("user.home").unwrap(),
                    ".local",
                    "share",
                    rel_plugins_path.to_str().unwrap(),
                ]
                .iter()
                .collect();
            }
            "macos" => {
                open_file_explorer_cmd = String::from("open");
                plugins_path = [
                    &env::var("user.home").unwrap(),
                    "Library",
                    "Application Support",
                    rel_plugins_path.to_str().unwrap(),
                ]
                .iter()
                .collect();
            }
            "windows" => {
                open_file_explorer_cmd = String::from("explorer");
                plugins_path = [
                    r"C:\",
                    "Program Files (x86)",
                    rel_plugins_path.to_str().unwrap(),
                ]
                .iter()
                .collect();
            }
            _ => panic!("OS not supported."),
        }
        (
            App {
                current_download: Download::new(0, String::from("")),
                install_buttons: Vec::new(),
                open_file_explorer_cmd: open_file_explorer_cmd,
                plugins_path: plugins_path,
                show_plugins: button::State::new(),
                theme: style::Theme::Dark,
            },
            Command::none(),
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        self.current_download.subscription()
    }

    fn title(&self) -> String {
        String::from("Crowkart Mod Installer")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Install(link) => {
                self.current_download = Download::new(0, link.clone());
                self.current_download.start();
                // let response = reqwest::blocking::get(link).unwrap();
                // let content = response.bytes().unwrap();
                // let reader = std::io::Cursor::new(content);
                // let zip = unzip::Unzipper::new(reader, &self.plugins_path);
                // zip.unzip().expect("Unable to unzip file");
            }
            Message::DownloadProgressed(progress) => {
                self.current_download.progress(progress.1);
            }
            Message::ShowPlugins => {
                Cmd::new(&self.open_file_explorer_cmd)
                    .arg(&self.plugins_path)
                    .spawn()
                    .unwrap();
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let mods_url = "https://raw.githubusercontent.com/jngo102/CKModInstaller/main/mods.json";
        let response = reqwest::blocking::get(mods_url).unwrap();
        let list = response
            .json::<HashMap<String, HashMap<String, String>>>()
            .unwrap();

        self.install_buttons.clear();
        for (mod_name, details) in list {
            self.install_buttons
                .push((mod_name, details, button::State::new()));
        }

        let mut column = self.install_buttons.iter_mut().fold(
            Column::new(),
            |col, (mod_name, details, state)| {
                col.push(
                    Column::new()
                        .push(
                            Row::new()
                                .push(Text::new(format!(
                                    "{} by {}",
                                    mod_name.to_string(),
                                    details["Author"].to_string()
                                )))
                                .push(
                                    Button::new(state, Text::new("Install"))
                                        .on_press(Message::Install(details["Link"].to_string()))
                                        .style(style::Theme::Dark),
                                )
                                .width(Length::Fill),
                        )
                        .push(Text::new(details["Description"].to_string()))
                        .padding(16)
                        .height(Length::Fill),
                )
            },
        );

        // let downloads_column = self.downloads.iter_mut().fold(Column::new().spacing(20), |column, download| {
        //     column.push(download.view())
        // })
        // .align_items(Align::End);

        let current_progress = match &self.current_download.state {
            State::Idle { .. } => 0.0,
            State::Downloading { progress } => *progress,
            State::Finished { .. } => 100.0,
            State::Errored { .. } => 0.0,
        };

        column = column
            .push(
                Button::new(
                    &mut self.show_plugins,
                    Text::new("Show Plugins").horizontal_alignment(HorizontalAlignment::Center),
                )
                .on_press(Message::ShowPlugins)
                .style(self.theme)
                .width(Length::Fill),
            )
            .push(
                Text::new("Icon by @liszhuk")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Bottom),
            // )
            // .push(
            //     downloads_column
            ).
            push(
                ProgressBar::new(0.0..=100.0, current_progress)
            );

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .padding(16)
            .style(self.theme)
            .into()
    }
}

#[derive(Debug)]
struct Download {
    id: usize,
    state: State,
    url: String,
}

#[derive(Debug)]
enum State {
    Idle { button: button::State },
    Downloading { progress: f32 },
    Finished { button: button::State },
    Errored { button: button::State },
}

impl Download {
    pub fn new(id: usize, url: String) -> Self {
        Download {
            id,
            state: State::Idle {
                button: button::State::new(),
            },
            url: url,
        }
    }

    pub fn start(&mut self) {
        match self.state {
            State::Idle { .. }
            | State::Finished { .. }
            | State::Errored { .. } => {
                self.state = State::Downloading { progress: 0.0 };
            }
            _ => {}
        }
    }

    pub fn progress(&mut self, new_progress: download::Progress) {
        match &mut self.state {
            State::Downloading { progress } => match new_progress {
                download::Progress::Started => {
                    *progress = 0.0;
                }
                download::Progress::Advanced(percentage) => {
                    *progress = percentage;
                }
                download::Progress::Finished => {
                    self.state = State::Finished {
                        button: button::State::new(),
                    }
                }
                download::Progress::Errored => {
                    self.state = State::Errored {
                        button: button::State::new(),
                    };
                }
            },
            _ => {}
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Downloading { .. } => {
                download::file(self.id, self.url.clone())
                    .map(Message::DownloadProgressed)
            }
            _ => Subscription::none(),
        }
    }

    // pub fn view(&mut self) -> Element<Message> {
    //     let current_progress = match &self.state {
    //         State::Idle { .. } => 0.0,
    //         State::Downloading { progress } => *progress,
    //         State::Finished { .. } => 100.0,
    //         State::Errored { .. } => 0.0,
    //     };

    //     let progress_bar = ProgressBar::new(0.0..=100.0, current_progress);

    //     Column::new()
    //         .spacing(10)
    //         .padding(10)
    //         .align_items(Align::Center)
    //         .push(progress_bar)
    //         .into()
    // }
}

mod style {
    use iced::{
        button, checkbox, container, progress_bar, radio, rule, scrollable, slider, text_input,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Theme {
        Light,
        Dark,
    }

    impl Theme {
        pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];
    }

    impl Default for Theme {
        fn default() -> Theme {
            Theme::Light
        }
    }

    impl From<Theme> for Box<dyn container::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Container.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn radio::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Radio.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn text_input::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::TextInput.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn button::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => light::Button.into(),
                Theme::Dark => dark::Button.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn scrollable::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Scrollable.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn slider::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Slider.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::ProgressBar.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn checkbox::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Checkbox.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn rule::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Rule.into(),
            }
        }
    }

    mod light {
        use iced::{button, Color, Vector};

        pub struct Button;

        impl button::StyleSheet for Button {
            fn active(&self) -> button::Style {
                button::Style {
                    background: Color::from_rgb(0.11, 0.42, 0.87).into(),
                    border_radius: 12.0,
                    shadow_offset: Vector::new(1.0, 1.0),
                    text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                    ..button::Style::default()
                }
            }

            fn hovered(&self) -> button::Style {
                button::Style {
                    text_color: Color::WHITE,
                    shadow_offset: Vector::new(1.0, 2.0),
                    ..self.active()
                }
            }
        }
    }

    mod dark {
        use iced::{
            button, checkbox, container, progress_bar, radio, rule, scrollable, slider, text_input,
            Color,
        };

        const SURFACE: Color = Color::from_rgb(
            0x40 as f32 / 255.0,
            0x44 as f32 / 255.0,
            0x4B as f32 / 255.0,
        );

        const ACCENT: Color = Color::from_rgb(
            0x6F as f32 / 255.0,
            0xFF as f32 / 255.0,
            0xE9 as f32 / 255.0,
        );

        const ACTIVE: Color = Color::from_rgb(
            0x72 as f32 / 255.0,
            0x89 as f32 / 255.0,
            0xDA as f32 / 255.0,
        );

        const HOVERED: Color = Color::from_rgb(
            0x67 as f32 / 255.0,
            0x7B as f32 / 255.0,
            0xC4 as f32 / 255.0,
        );

        pub struct Container;

        impl container::StyleSheet for Container {
            fn style(&self) -> container::Style {
                container::Style {
                    background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
                    text_color: Color::WHITE.into(),
                    ..container::Style::default()
                }
            }
        }

        pub struct Radio;

        impl radio::StyleSheet for Radio {
            fn active(&self) -> radio::Style {
                radio::Style {
                    background: SURFACE.into(),
                    dot_color: ACTIVE,
                    border_width: 1.0,
                    border_color: ACTIVE,
                }
            }

            fn hovered(&self) -> radio::Style {
                radio::Style {
                    background: Color { a: 0.5, ..SURFACE }.into(),
                    ..self.active()
                }
            }
        }

        pub struct TextInput;

        impl text_input::StyleSheet for TextInput {
            fn active(&self) -> text_input::Style {
                text_input::Style {
                    background: SURFACE.into(),
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                }
            }

            fn focused(&self) -> text_input::Style {
                text_input::Style {
                    border_width: 1.0,
                    border_color: ACCENT,
                    ..self.active()
                }
            }

            fn hovered(&self) -> text_input::Style {
                text_input::Style {
                    border_width: 1.0,
                    border_color: Color { a: 0.3, ..ACCENT },
                    ..self.focused()
                }
            }

            fn placeholder_color(&self) -> Color {
                Color::from_rgb(0.4, 0.4, 0.4)
            }

            fn value_color(&self) -> Color {
                Color::WHITE
            }

            fn selection_color(&self) -> Color {
                ACTIVE
            }
        }

        pub struct Button;

        impl button::StyleSheet for Button {
            fn active(&self) -> button::Style {
                button::Style {
                    background: ACTIVE.into(),
                    border_radius: 3.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                }
            }

            fn hovered(&self) -> button::Style {
                button::Style {
                    background: HOVERED.into(),
                    text_color: Color::WHITE,
                    ..self.active()
                }
            }

            fn pressed(&self) -> button::Style {
                button::Style {
                    border_width: 1.0,
                    border_color: Color::WHITE,
                    ..self.hovered()
                }
            }
        }

        pub struct Scrollable;

        impl scrollable::StyleSheet for Scrollable {
            fn active(&self) -> scrollable::Scrollbar {
                scrollable::Scrollbar {
                    background: SURFACE.into(),
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    scroller: scrollable::Scroller {
                        color: ACTIVE,
                        border_radius: 2.0,
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            }

            fn hovered(&self) -> scrollable::Scrollbar {
                let active = self.active();

                scrollable::Scrollbar {
                    background: Color { a: 0.5, ..SURFACE }.into(),
                    scroller: scrollable::Scroller {
                        color: HOVERED,
                        ..active.scroller
                    },
                    ..active
                }
            }

            fn dragging(&self) -> scrollable::Scrollbar {
                let hovered = self.hovered();

                scrollable::Scrollbar {
                    scroller: scrollable::Scroller {
                        color: Color::from_rgb(0.85, 0.85, 0.85),
                        ..hovered.scroller
                    },
                    ..hovered
                }
            }
        }

        pub struct Slider;

        impl slider::StyleSheet for Slider {
            fn active(&self) -> slider::Style {
                slider::Style {
                    rail_colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
                    handle: slider::Handle {
                        shape: slider::HandleShape::Circle { radius: 9.0 },
                        color: ACTIVE,
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            }

            fn hovered(&self) -> slider::Style {
                let active = self.active();

                slider::Style {
                    handle: slider::Handle {
                        color: HOVERED,
                        ..active.handle
                    },
                    ..active
                }
            }

            fn dragging(&self) -> slider::Style {
                let active = self.active();

                slider::Style {
                    handle: slider::Handle {
                        color: Color::from_rgb(0.85, 0.85, 0.85),
                        ..active.handle
                    },
                    ..active
                }
            }
        }

        pub struct ProgressBar;

        impl progress_bar::StyleSheet for ProgressBar {
            fn style(&self) -> progress_bar::Style {
                progress_bar::Style {
                    background: SURFACE.into(),
                    bar: ACTIVE.into(),
                    border_radius: 10.0,
                }
            }
        }

        pub struct Checkbox;

        impl checkbox::StyleSheet for Checkbox {
            fn active(&self, is_checked: bool) -> checkbox::Style {
                checkbox::Style {
                    background: if is_checked { ACTIVE } else { SURFACE }.into(),
                    checkmark_color: Color::WHITE,
                    border_radius: 2.0,
                    border_width: 1.0,
                    border_color: ACTIVE,
                }
            }

            fn hovered(&self, is_checked: bool) -> checkbox::Style {
                checkbox::Style {
                    background: Color {
                        a: 0.8,
                        ..if is_checked { ACTIVE } else { SURFACE }
                    }
                    .into(),
                    ..self.active(is_checked)
                }
            }
        }

        pub struct Rule;

        impl rule::StyleSheet for Rule {
            fn style(&self) -> rule::Style {
                rule::Style {
                    color: SURFACE,
                    width: 2,
                    radius: 1.0,
                    fill_mode: rule::FillMode::Padded(15),
                }
            }
        }
    }
}

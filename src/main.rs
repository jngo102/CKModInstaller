//#![windows_subsystem = "windows"]

use iced::{
    button, executor, Application, Button, Clipboard, Column, Command, Container, Element, Length,
    Row, Settings, Text,
};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::Command as Cmd;

fn main() -> iced::Result {
    let steam_path: PathBuf = ["Steam", "steamapps", "common", "Crowkart"]
        .iter()
        .collect();
    let (bepinex_url, crowkart_game, crowkart_path) = get_os_specific_values(steam_path);
    check_bepinex_install(bepinex_url, crowkart_game, crowkart_path)
        .expect("Could not check BepInEx installation.");

    App::run(Settings::default())
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
        let reader = std::io::Cursor::new(content);
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
    install_buttons: Vec<(String, HashMap<String, String>, button::State)>,
    open_file_explorer_cmd: String,
    plugins_path: PathBuf,
    show_plugins: button::State,
}

#[derive(Clone, Debug)]
enum Message {
    Install(String),
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
                install_buttons: Vec::new(),
                open_file_explorer_cmd: open_file_explorer_cmd,
                plugins_path: plugins_path,
                show_plugins: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Crowkart Mod Installer")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Install(link) => {
                let response = reqwest::blocking::get(link).unwrap();
                let content = response.bytes().unwrap();
                let reader = std::io::Cursor::new(content);
                let zip = unzip::Unzipper::new(reader, &self.plugins_path);
                zip.unzip().expect("Unable to unzip file");
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
                                    "{:?} by {:?}",
                                    mod_name.to_string(),
                                    details["Author"].to_string()
                                )))
                                .push(
                                    Button::new(state, Text::new("Install"))
                                        .on_press(Message::Install(details["Link"].to_string())),
                                ),
                        )
                        .push(Text::new(details["Description"].to_string())),
                )
            },
        );

        column = column.push(
            Button::new(&mut self.show_plugins, Text::new("Show Plugins"))
                .on_press(Message::ShowPlugins),
        );

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .padding(16)
            .into()
    }
}

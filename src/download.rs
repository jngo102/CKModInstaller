use std::env;
use iced_futures::futures;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

// Just a little utility function
pub fn file<I: 'static + Hash + Copy + Send, T: ToString>(
    id: I,
    url: T,
) -> iced::Subscription<(I, Progress)> {
    iced::Subscription::from_recipe(Download {
        id,
        url: url.to_string(),
    })
}

pub struct Download<I> {
    id: I,
    url: String,
}

impl<T> Download<T>
where
    T: 'static + Hash + Copy + Send,
{
    async fn save_to_file(response: reqwest::Response) {
        println!("Saving to file...");
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
        match env::consts::OS {
            "linux" => {
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

        let content = response.bytes().await;
        let reader = std::io::Cursor::new(content.unwrap());
        let zip = unzip::Unzipper::new(reader, &plugins_path);
        zip.unzip().expect("Unable to unzip file");
    }
}

// Make sure iced can use our download stream
impl<H, I, T> iced_futures::subscription::Recipe<H, I> for Download<T>
where
    T: 'static + Hash + Copy + Send,
    H: Hasher,
{
    type Output = (T, Progress);

    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.id.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        let id = self.id;

        Box::pin(futures::stream::unfold(
            State::Ready(self.url),
            move |state| async move {
                match state {
                    State::Ready(url) => {
                        let response = reqwest::get(&url).await;
                        match response {
                            Ok(response) => {
                                if let Some(total) = response.content_length() {
                                    Some((
                                        (id, Progress::Started),
                                        State::Downloading {
                                            response,
                                            total,
                                            downloaded: 0,
                                        },
                                    ))
                                } else {
                                    Some((
                                        (id, Progress::Errored),
                                        State::Finished { response: Option::from(response) },
                                    ))
                                }
                            }
                            Err(_) => {
                                Some(((id, Progress::Errored), State::Finished { response: None }))
                            }
                        }
                    }
                    State::Downloading {
                        mut response,
                        total,
                        downloaded,
                    } => match response.chunk().await {
                        Ok(Some(chunk)) => {
                            let downloaded = downloaded + chunk.len() as u64;

                            let percentage =
                                (downloaded as f32 / total as f32) * 100.0;

                            Some((
                                (id, Progress::Advanced(percentage)),
                                State::Downloading {
                                    response,
                                    total,
                                    downloaded,
                                },
                            ))
                        }
                        Ok(None) => {
                            Some(((id, Progress::Finished), State::Finished { response: Option::from(response) }))
                        }
                        Err(_) => {
                            Some(((id, Progress::Errored), State::Finished { response: Option::from(response) }))
                        }
                    },
                    State::Finished {
                        response,
                    }=> {
                        Download::<T>::save_to_file(response.unwrap()).await;

                        // We do not let the stream die, as it would start a
                        // new download repeatedly if the user is not careful
                        // in case of errors.
                        let _: () = iced::futures::future::pending().await;

                        None
                    }
                }
            },
        ))
    }
}

#[derive(Debug, Clone)]
pub enum Progress {
    Started,
    Advanced(f32),
    Finished,
    Errored,
}

pub enum State {
    Ready(String),
    Downloading {
        response: reqwest::Response,
        total: u64,
        downloaded: u64,
    },
    Finished {
        response: Option<reqwest::Response>,
    },
}
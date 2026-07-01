use arboard::Clipboard;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dirs::download_dir;
use rfd::FileDialog;
use std::path::PathBuf;
use youtube_dl::YoutubeDl;

fn main() {
    LaunchBuilder::new()
        .with_cfg(
            Config::new()
                .with_disable_context_menu(true)
                .with_menu(None)
                .with_window(WindowBuilder::new().with_title("videofromlink")),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut link = use_signal(|| String::new());
    let mut output = use_signal(|| PathBuf::new());

    //println!("{}, {:?}", link, output);
    //println!("{}", invalid_args(&link.read(), &output.read()));

    rsx! {
        tr {
            "link: ",
            input {
                value: link.read().as_str(),
                oninput: move |i| {link.set(i.value())},
                size: 50,
                placeholder: "https://...",
            },
            button {
                onclick: move |_| link.set(Clipboard::new().unwrap().get_text().unwrap()),
                "paste"
            }
        },
        tr {
            "output path: ",
            input {
                value: output.read().to_str(),
                oninput: move |i| {output.set(PathBuf::from(i.value()))},
                size: 50,
                placeholder: "saves to downloads by default"
            },
            button {
                onclick: move |_| output.set(dialog_handling(FileDialog::new().set_directory("/").pick_folder())),
                "select"
            }
        },
        tr {
            button {
                disabled: invalid_args(&link.read(), &output.read()).to_string(),
                onclick: move |_| {
                    let out = YoutubeDl::new(link.read().as_str())
                        .download_to(some_or_downloads(output.read().to_owned()).as_path())
                        .unwrap();

                    println!("{:?}", out);
                },
                "download"
            }
        }
    }
}

fn invalid_args(link: &str, output: &PathBuf) -> bool {
    if (output.exists() || output == &PathBuf::new()) && link.starts_with("https://") {
        false
    } else {
        true
    }
}

fn some_or_downloads(path: PathBuf) -> PathBuf {
    if path == PathBuf::new() {
        return download_dir().unwrap();
    } else {
        return path;
    }
}

fn dialog_handling(path: Option<PathBuf>) -> PathBuf {
    match path {
        Some(value) => return value,
        None => return PathBuf::new(),
    }
}

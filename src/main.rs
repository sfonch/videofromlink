use arboard::Clipboard;
use dioxus::{
    desktop::{Config, LogicalSize, WindowBuilder},
    prelude::*,
};
use dirs::download_dir;
//use image::ImageReader;
use rfd::FileDialog;
use std::path::PathBuf;
//use std::sync::LazyLock;
use youtube_dl::YoutubeDl;

// static ICON: LazyLock<Icon> = LazyLock::new(|| icon());
const STYLE: Asset = asset!("src/style.css");

const EMPTYPATH: PathBuf = PathBuf::new();

const REMUX: &str = "--remux-video";
const RECODE: &str = "--recode-video";

fn main() {
    LaunchBuilder::new()
        .with_cfg(
            Config::new()
                //                .with_icon(ICON.to_owned())
                .with_disable_context_menu(true)
                .with_menu(None)
                .with_window(
                    WindowBuilder::new()
                        .with_title("videofromlink")
                        .with_inner_size(LogicalSize::new(960, 720)),
                ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut link = use_signal(|| String::new());
    let mut output = use_signal(|| EMPTYPATH);
    let mut format = use_signal(|| String::from("mp4"));

    rsx! {
        document::Stylesheet { href: STYLE }
        tr {
            id: "d",
            class: "t",
            "Link: ",
            input {
                class: "i",
                value: link.read().as_str(),
                oninput: move |i| {link.set(i.value())},
                size: 50,
                placeholder: "https://...",
            },
            button {
                class: "b",
                onclick: move |_| link.set(Clipboard::new().unwrap().get_text().unwrap()),
                "Paste"
            }
        },
        tr {
            id: "d",
            class: "t",
            "Output path: ",
            input {
                class: "i",
                value: output.read().to_str(),
                oninput: move |i| {output.set(PathBuf::from(i.value()))},
                size: 50,
                placeholder: "Saves to downloads by default"
            },
            button {
                class: "b",
                onclick: move |_| output.set(dialog_handling(FileDialog::new().set_directory("/").pick_folder())),
                "Select"
            }
        },
        tr {
            id: "d",
            button {
                class: "b",
                disabled: invalid_args(&link.read(), &output.read()).to_string(),
                onclick: move |_| {
                    let out = YoutubeDl::new(link.read().as_str())
                        .extra_arg(remux_or_recode(&format.read()))
                        .extra_arg(format!("{format}"))
                        .download_to(some_or_downloads(output.read().to_owned()).as_path());

                    println!("{:?}", out);
                },
                "Download"
            },
            select {
                value: "{format}",
                onchange: move |v| {
                    format.set(v.value());
                },

                option { value: "mp4", "mp4" },
                option { value: "mkv", "mkv" },
                option { value: "mov", "mov" },
                option { value: "avi", "avi" },
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
    if path == EMPTYPATH {
        return download_dir().unwrap();
    } else {
        return path;
    }
}

fn dialog_handling(path: Option<PathBuf>) -> PathBuf {
    match path {
        Some(value) => return value,
        None => return EMPTYPATH,
    }
}

fn remux_or_recode(format: &str) -> &str {
    match format {
        "mp4" => REMUX,
        "mkv" => REMUX,
        "mov" => RECODE,
        "avi" => RECODE,
        _ => panic!(),
    }
}

/*
fn icon() -> Icon {
    let img = ImageReader::open("/home/sfon/Documents/Rust/videofromlink/src/icon.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();

    Icon::from_rgba(img.to_vec(), img.width(), img.height()).unwrap()
}
*/

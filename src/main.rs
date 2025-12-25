use dioxus::prelude::*;
use reqwest;
// use rusqlite;
use serde;

static CSS: Asset = asset!("/assets/main.css");

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div {
            id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });
    // let mut img_src = use_resource(snipped!());

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button {
                id: "save",
                onclick: move |_| async move {
                    let current = img_src.cloned().unwrap();
                    img_src.restart();
                    _ = save_dog(current).await;
                },
                "save!"
            }
        }
    }
}

#[server]
async fn save_dog(image: String) -> Result<()> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database.");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );"
        ).unwrap();
        conn
    };
}

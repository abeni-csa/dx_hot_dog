use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/main.css");
// static ICON: Asset = asset!("/assets/icon.ico", ImageAssetOptions::new().with_avif());
fn main() {
    dioxus::launch(App);
}
#[derive(Clone)]
struct TitleState(String);

#[derive(Clone, Copy)]
struct MusicPlayer {
    song: Signal<String>,
}

fn use_music_player_provider() {
    let song = use_signal(|| "Drift Away".to_string());
    use_context_provider(|| MusicPlayer { song });
}

#[component]
fn App() -> Element {
    use_context_provider(|| TitleState("HOT DOG".to_string()));
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
        Player { }
    }
}
#[component]
fn Player() -> Element {
    rsx! {
        button {
            onclick: move |_| consume_context::<MusicPlayer>().song.set("Vienna".to_string()),
            "Shuffle"
         }
    }
}
#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        h1 { "{title.0}" }
    }
}
#[component]
fn DogView() -> Element {
    let mut img_src = use_signal(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    let skip = move |env| {};
    let save = move |env| {
        img_src.set("https://images.dog.ceo/breeds/briard/n02105251_6984.jpg");
    };
    rsx! {
        div {

             id: "dogview",
            img { src: "{img_src}"}

         }
         div { id: "buttons",
             button { onclick:skip, id: "save", "Save" }
             button { onclick:save, id: "skip", "Skip" }
         }

    }
}

use dioxus::prelude::*;
static SONG: GlobalSignal<String> = Signal::global(|| "Drift Away".to_string());
static CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::launch(App);
}
#[derive(Clone)]
struct TitleState(String);
#[derive(serde::Deserialize)]
struct DogAPI {
    message: String,
}

#[component]
fn App() -> Element {
    use_context_provider(|| TitleState("HOT DOG".to_string()));
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
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
    let mut img_src = use_signal(|| "".to_string());
    let save = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogAPI>()
            .await
            .unwrap();
        img_src.set(response.message);
    };
    rsx! {
        div {

             id: "dogview",
            img { src: "{img_src}"}

         }
         div { id: "buttons",
             button { onclick:save, id: "save", "Priv" }
             button { onclick:save, id: "skip", "Next" }
         }

    }
}

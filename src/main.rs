use dioxus::prelude::*;
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
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogAPI>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div {

             id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }

         }
         div { id: "buttons",
             button { onclick: move |_| img_src.restart(), id: "save", "Priv" }
             button { onclick: move |_| img_src.restart(), id: "skip", "Next" }
         }

    }
}
#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    Ok(())
}

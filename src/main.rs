use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/main.css");
static ICON: Asset = asset!("/assets/icon.ico", ImageAssetOptions::new().with_avif());
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{ href: CSS }
        div {
             id: "title",
              h1 { "Hot Dog" }
          }
        div {

             id: "dogview",
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg"}

         }
         div { id: "buttons",
             button { id: "save", "Save" }
             button { id: "skip", "Skip" }
         }

    }
}

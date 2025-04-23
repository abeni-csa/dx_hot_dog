use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/main.css");
// static ICON: Asset = asset!("/assets/icon.ico", ImageAssetOptions::new().with_avif());
fn main() {
    dioxus::launch(App);
}

#[component]
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
     div {
         id: "title",
         h1 { "HOT DOG" }
      }
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

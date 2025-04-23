use dioxus::prelude::*;
use global_attributes::user_select;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
             id: "title",
              h1 { "Hot Dog" }
          }
        div {
             id: "dogpic",
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
         }
         div { id: "buttons",
             button { id: "save", "Save" }
             button { id: "skip", "Skip" }
         }

    }
}

use dioxus::prelude::*;
use global_attributes::user_select;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
         div {
            color: "red",
            "Front end with out Js!",
            button { "Click me!"}
        }
    };
    rsx! {
         div {  color: "green", "somethings" }
        ol {
            {(0..10).map(|i| rsx!{ "{i} [name]" })}
         }
    };
    let show_titile = true;
    let users = vec!["Abeni", "nahom", "nati", "Jemil", "abi"];
    rsx! {
        if show_titile {
            h1 { "Hellow world" }
        }
        for item in 0..10 {
             ol {
                 "{item}"
             }
        }
        for ( id,name ) in users.iter().enumerate() {
            div {
                ol {
                " [!] {id}",
                " [+] {name}"
            }

            }
        }
    }
}
// rsx! {
//     // Anything that's `Display`
//     {"Something"}
//
//     // Optionals
//     {show_title.then(|| rsx! { "title!" } )}
//
//     // And iterators
//     ul {
//         {(0..5).map(|i| rsx! { "{i}" })}
//     }
// }

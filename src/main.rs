#![allow(non_snake_case)]
use dioxus::prelude::*;

fn App(cx: Scope) -> Element {
    let on_key_down_handler = move |e: KeyboardEvent| {
        let code = e.code();
        println!("Event: {code:?}");
        // log::info!("{index}{}", evt.key);
        // match evt.key.as_str() {
        //     "ArrowRight" => index += 1,
        //     "ArrowLeft" => index -= 1,
        //     _ => {}
        // }
    };

    cx.render(rsx! {
        style { include_str!("../assets/main.css") }
        div {
            width: "100vw",
            height: "100vh",
            background: "red",
            tabindex: "1",
            onkeydown: on_key_down_handler,
        }
    })
}

fn test_component(cx: Scope) -> Element {
    cx.render(rsx! {
        div {}
    })
}

fn main() {
    dioxus_desktop::launch(App);
}

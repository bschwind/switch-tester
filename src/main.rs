#![allow(non_snake_case)]

use crate::key_switch::{KeySwitch, SwitchData};
use dioxus::prelude::*;
use dioxus_desktop::Config;

mod key_switch;

fn App(cx: Scope<SwitchData>) -> Element {
    let current_switch = use_state(cx, || None::<KeySwitch>);

    let on_key_down_handler = move |e: KeyboardEvent| {
        let code = e.code();

        if let Some(switch_name) = cx.props.map.get(&code) {
            if let Some(key_switch) = cx.props.switches.get(switch_name) {
                current_switch.set(Some(key_switch.clone()));
            }
        }
    };

    cx.render(rsx! {
        style { include_str!("../assets/main.css") }
        div {
            class: "main-container",
            tabindex: "1",
            autofocus: true,
            onkeydown: on_key_down_handler,

            if let Some(switch) = current_switch.get() {
                rsx!{
                    KeySwitchInfo {
                        switch: switch
                    }
                }
            } else {
                rsx! {
                    span {
                        "Press a key to see its switch details!"
                    }
                }
            }
        }
    })
}

#[derive(Props)]
struct KeySwitchProps<'a> {
    switch: &'a KeySwitch,
}

fn KeySwitchInfo<'a>(cx: Scope<'a, KeySwitchProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            img {
                class: "switch-image",
                src: "{cx.props.switch.image_url}",
            }
            h2 {
                "{cx.props.switch.name}"
            }
        }
    })
}

fn main() -> anyhow::Result<()> {
    let switch_json = std::fs::read_to_string("assets/switch_map.json5")?;
    let switch_data: SwitchData = json5::from_str(&switch_json)?;

    dioxus_desktop::launch_with_props(App, switch_data, Config::default());

    Ok(())
}

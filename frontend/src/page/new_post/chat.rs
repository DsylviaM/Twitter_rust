#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::{html::button, prelude::*};

pub fn NewChat(cx:Scope) -> Element {
    cx.render(rsx! {
        form {
            class: "flex flex-col gap-4",
            onsubmit: move |_| (),
            //massege input
            //headline input
            button {
                class: "btn",
                r#type: "submit",
                disabled: true,
                "Post"

            }
            tr{"g"}tr{"g"}tr{"g"}tr{"g"}
        }
    })
}
#![allow(non_snake_case)]

use crate::{elements::toaster, prelude::*, util::api_client};
use chrono::Duration;
use dioxus::prelude::*;
use uchat_domain::{ids::PostId, post::Message};
use uchat_endpoint::post::types::NewPostOptions;

fn can_submit(message: &str) -> bool {
    message.len() < Message::MAX_CHARS && !message.is_empty()
}

#[inline_props]
pub fn QuickRespond(cx: Scope, post_id: PostId, opened: UseState<bool>) -> Element {
    let api_client = ApiClient::global();
    let toaster = use_toaster(cx);

    let message = use_state(cx, || "".to_string());

    let form_onsubmit = async_handler!(
        &cx,
        [api_client, toaster],
        move |_| async move {
        use uchat_domain::post::{ Message};
        use uchat_endpoint::post::endpoint::{NewPost, NewPostOk};
        use uchat_endpoint::post::types::{Chat};

        let request = NewPost {
            content: Chat {
                headline: None,
                message: Message::new(message.get()).unwrap(),
            }
            .into(),
            options: NewPostOptions::default(),
        };
        let response = fetch_json!(<NewPostOk>, api_client, request);
        match response {
            Ok(_) => {
                toaster.write().succsess("Posted!", Duration::seconds(3));
                opened.set(false);
            }
            Err(e) => {
                toaster
                    .write()
                    .error(format!("Reply failed: {e}"), Duration::seconds(3));
            },
            
        }
    });

    let submit_cursor = if can_submit(message.get()) {
        "cursor-pointer"
    } else {
        "cursor-not-allowed"
    };

    cx.render(rsx! {
        form {
            onsubmit: form_onsubmit,
            prevent_default: "onsubmit",
            //messege
            div {
                class: "w-full flex-row justify-end",
                button {
                    class: "mt-2 btn",
                    r#type: "submit",
                    disabled: !can_submit(message.get()),
                    "Respond"
                }
            }
        }
    })
}
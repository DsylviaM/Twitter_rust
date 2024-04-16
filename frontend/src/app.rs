#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::{elements::Navbar, prelude::*};

use dioxus_router::{Route, Router};
use fermi::use_init_atom_root;



pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    cx.render (rsx! {
        Router {
             Route{ to: page::ACCOUNT_REGISTER, page::Register {} },
             Route{ to: page::ACCOUNT_LOGIN, page::Login {} },
             Route{ to: page::HOME, page::Home {} },
             Route{ to: page::POST_NEW_CHAT, page::NewChat {}},
             Navbar{}
        }
    })
}

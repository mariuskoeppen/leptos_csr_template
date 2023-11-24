#![allow(unused)]
use leptos::*;
use leptos_csr_template::{utils::*, *};

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> });
}


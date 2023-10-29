#![feature(iter_array_chunks)]

mod widgets;

use leptos::*;
use widgets::App;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

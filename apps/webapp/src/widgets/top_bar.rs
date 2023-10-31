use leptos::*;

#[component]
pub fn TopBar() -> impl IntoView {
    view! {
        <div style="grid-area: top" class="bg-blue-950 text-white flex justify-around items-center">
            <span>eeric</span>
            <span>v{move || env!("CARGO_PKG_VERSION")}</span>
        </div>
    }
}

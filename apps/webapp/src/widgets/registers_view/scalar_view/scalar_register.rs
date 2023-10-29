use leptos::*;

#[component]
pub fn ScalarRegister(cx: Scope, name: String, value: String) -> impl IntoView {
    view! { cx,
        <div class="border border-gray-300 w-20 divide-y flex flex-col justify-center bg-white overflow-hidden whitespace-nowrap text-ellipsis">
            <div class="text-center font-bold">{value}</div>
            <div class="text-center bg-gray-100">{name}</div>
        </div>
    }
}

use std::collections::VecDeque;

use eeric_core::prelude::*;
use leptos::*;

use crate::widgets::global_state;

#[component]
pub fn MemoryView() -> impl IntoView {
    let core = expect_context::<RwSignal<global_state::Machine>>();
    let memory = create_read_slice(core, |state| {
        state
            .read_core()
            .map(|machine| machine.memory.snapshot())
            .unwrap_or(VecDeque::from_iter((0..0x100).map(|_| 0)))
    });

    view! {
    <div class="bg-white rounded p-4 shadow-xl max-h-[75%] overflow-y-scroll">
        <h1 class="font-bold text-center border border-gray-200 py-6">Memory</h1>
        <div class="grid grid-cols-[repeat(17,minmax(0,max-content))] px divide-x divide-y border border-gray-200 text-center">
            <div class="bg-gray-100 font-bold">+</div>
            {
                (0..0x10).map(|i| format!("{:x}", i)).map(|addr| view! {<div class="w-10 bg-gray-100 font-bold">{addr.to_uppercase()}</div>}).collect_view()
            }
            {move || {
                memory()
                    .iter()
                    .enumerate()
                    .map(|(address, byte)| {
                        view! {
                            <>
                                <Show
                                    when=move || address % 0x10 == 0
                                    fallback=|| ()
                                >
                                    <div class="w-12 bg-gray-100 font-bold">{format!("{:02x}", address).to_uppercase()}</div>
                                </Show>
                                <div>
                                    {byte.to_string()}
                                </div>
                            <>
                        }
                    })
                    .collect_view()
            }}
        </div>
    </div> }
}

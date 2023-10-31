use eeric_core::prelude::*;
use leptos::*;

use crate::widgets::global_state;

use super::scalar_register::ScalarRegister;

#[component]
pub fn PcRegister() -> impl IntoView {
    let core = expect_context::<RwSignal<global_state::Machine>>();
    let pc = create_read_slice(core, |state| {
        state
            .read_core()
            .map(|machine| machine.registers.snapshot().pc)
            .unwrap_or_default()
    });

    view! {
        <div class="text-center bg-white rounded p-4 shadow-xl">
            {move || view! { <ScalarRegister name=String::from("pc") value=pc().to_string()/> }}

        </div>
    }
}

use eeric_core::prelude::*;
use leptos::*;

use crate::widgets::global_state;

#[component]
pub fn CsrView() -> impl IntoView {
    let core = expect_context::<RwSignal<global_state::Machine>>();

    let (prompt, set_prompt) = create_signal("".to_owned());

    let regs = create_read_slice(core, |state| {
        state
            .read_core()
            .map(|machine| machine.registers.snapshot())
            .unwrap_or_default()
    });

    view! {
        <>
            <input
                type="text"
                prop:placeholder="Search for CSR register..."
                on:input=move |ev| { set_prompt(event_target_value(&ev)) }

                class="rounded w-3/4"
            />
            <div class="bg-white rounded p-4 shadow-xl max-h-[75%] overflow-y-scroll">
                <h1 class="font-bold text-center border border-gray-200 py-6">CSR registers</h1>
                <div class="grid grid-cols-[repeat(4,minmax(0,max-content))] px divide-x divide-y border border-gray-200">
                    <div class="font-bold text-center bg-gray-200 px-2">Name</div>
                    <div class="font-bold text-center bg-gray-200 px-2">Writable?</div>
                    <div class="font-bold text-center bg-gray-200 px-2">Bit representation</div>
                    <div class="font-bold text-center bg-gray-200 px-2">Number representation</div>
                    {move || {
                        regs()
                            .c
                            .iter()
                            .enumerate()
                            .filter_map(|(index, value)| csr_name(index).map(|name| (name, value)))
                            .filter(|(name, _)| { name.contains(&prompt()) })
                            .map(|(name, csr)| {
                                let is_writable = csr.privilege == CsrPrivilege::ReadWrite;
                                view! {
                                    <>
                                        <div class="px-2 text-center">{name}</div>
                                        <div class="px-2 text-center"
                                            class=("bg-green-100", is_writable)
                                            class=("bg-red-100", !is_writable)
                                        >{if is_writable { "Yes" } else { "No" }}</div>
                                        <div class="px-2 text-right font-mono">
                                            {format!("{:#064b}", csr.read())}
                                        </div>
                                        <div class="px-2 text-right font-mono">
                                            {format!("{}", csr.read())}
                                        </div>
                                    </>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </>
    }
}

fn csr_name(index: usize) -> Option<&'static str> {
    let csr = match index {
        0x0c02 => "instret",
        0x0c00 => "cycle",
        0x0c01 => "time",
        0x0f12 => "marchid",
        0x0003 => "fcsr",
        0x0001 => "fflags",
        0x0002 => "frm",
        0x0300 => "mstatus",
        0x0200 => "vsstatus",
        0x0C20 => "vl",
        0x0C21 => "vtype",
        0x0C22 => "vlenb",
        0x0008 => "vstart",
        0x000A => "vxrm",
        0x0009 => "vxsat",
        0x000F => "vcsr",
        _ => return None,
    };

    Some(csr)
}

use eeric_core::prelude::*;
use leptos::*;

use crate::widgets::global_state;

use super::scalar_register::ScalarRegister;

fn xreg_name(index: usize) -> String {
    match index {
        0 => "x0(zero)",
        1 => "x1(ra)",
        2 => "x2(sp)",
        3 => "x3(gp)",
        4 => "x4(tp)",
        5 => "x5(t0)",
        6 => "x6(t1)",
        7 => "x7(t2)",
        8 => "x8(s0/fp)",
        9 => "x9(s1)",
        10 => "x10(a0)",
        11 => "x11(a1)",
        12 => "x12(a2)",
        13 => "x13(a3)",
        14 => "x14(a4)",
        15 => "x15(a5)",
        16 => "x16(a6)",
        17 => "x17(a7)",
        18 => "x18(s2)",
        19 => "x19(s3)",
        20 => "x20(s4)",
        21 => "x21(s5)",
        22 => "x22(s6)",
        23 => "x23(s7)",
        24 => "x24(s8)",
        25 => "x25(s9)",
        26 => "x26(s10)",
        27 => "x27(s11)",
        28 => "x28(t3)",
        29 => "x29(t4)",
        30 => "x30(t5)",
        31 => "x31(t6)",
        _ => "?",
    }
    .to_owned()
}

#[component]
pub fn IntegerRegisters(cx: Scope) -> impl IntoView {
    let core = expect_context::<RwSignal<global_state::Machine>>(cx);
    let xregs = create_read_slice(cx, core, |state| {
        state
            .read_core()
            .map(|machine| machine.registers.snapshot().x)
            .unwrap_or_default()
    });

    view! { cx,
        <div class="text-center bg-white rounded p-4 shadow-xl">
            <h1 class="font-bold text-center border border-gray-200 p-6">Integer registers</h1>
            <div class="grid grid-cols-8 justify-items-center">
                {move || {
                    xregs()
                        .into_iter()
                        .enumerate()
                        .map(|(index, value)| {
                            view! { cx,
                                <ScalarRegister name=xreg_name(index) value=value.to_string()/>
                            }
                        })
                        .collect::<Vec<_>>()
                }}

            </div>
        </div>
    }
}

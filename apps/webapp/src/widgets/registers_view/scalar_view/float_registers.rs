use eeric_core::prelude::*;
use leptos::*;

use crate::widgets::global_state;

use super::scalar_register::ScalarRegister;

#[component]
pub fn FloatRegisters(cx: Scope) -> impl IntoView {
    let core = expect_context::<RwSignal<global_state::Machine>>(cx);
    let fregs = create_read_slice(cx, core, |state| {
        state
            .read_core()
            .map(|machine| machine.registers.snapshot().f)
            .unwrap_or_default()
    });

    view! { cx,
        <div class="text-center bg-white rounded p-4 shadow-xl">
            <h1 class="font-bold text-center border border-gray-200 p-6">Float registers</h1>
            <div class="grid grid-cols-8 justify-items-center">
                {move || {
                    fregs()
                        .into_iter()
                        .enumerate()
                        .map(|(index, value)| {
                            view! { cx,
                                <ScalarRegister name=freg_name(index) value=format!("{:.2}", value)/>
                            }
                        })
                        .collect::<Vec<_>>()
                }}

            </div>
        </div>
    }
}

fn freg_name(index: usize) -> String {
    match index {
        0 => "f0(ft0)",
        1 => "f1(ft1)",
        2 => "f2(ft2)",
        3 => "f3(ft3)",
        4 => "f4(ft4)",
        5 => "f5(ft5)",
        6 => "f6(ft6)",
        7 => "f7(ft7)",
        8 => "f8(fs0)",
        9 => "f9(fs1)",
        10 => "f10(fa0)",
        11 => "f11(fa1)",
        12 => "f12(fa2)",
        13 => "f13(fa3)",
        14 => "f14(fa4)",
        15 => "f15(fa5)",
        16 => "f16(fa6)",
        17 => "f17(fa7)",
        18 => "f18(fs2)",
        19 => "f19(fs3)",
        20 => "f20(fs4)",
        21 => "f21(fs5)",
        22 => "f22(fs6)",
        23 => "f23(fs7)",
        24 => "f24(fs8)",
        25 => "f25(fs9)",
        26 => "f26(fs10)",
        27 => "f27(fs11)",
        28 => "f28(ft8)",
        29 => "f29(ft9)",
        30 => "f30(ft10)",
        31 => "f31(ft11)",
        _ => "?",
    }
    .to_owned()
}

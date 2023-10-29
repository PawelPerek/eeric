mod float_registers;
mod integer_registers;
mod pc_register;
mod scalar_register;

use leptos::*;

use float_registers::FloatRegisters;
use integer_registers::IntegerRegisters;
use pc_register::PcRegister;

#[component]
pub fn ScalarView(cx: Scope) -> impl IntoView {
    view! { cx,
        <>
            <PcRegister/>
            <IntegerRegisters/>
            <FloatRegisters/>
        </>
    }
}

mod editor;
mod example;
mod top_bar;

use editor::Editor;
use eeric_core::prelude::*;
use eeric_interpreter::prelude::*;
use example::Example;
use leptos::{leptos_dom::logging::console_log, *};
use top_bar::TopBar;

use crate::widgets::global_state;

#[component]
pub fn ProgramView() -> impl IntoView {
    let core = expect_context::<RwSignal<global_state::Machine>>();
    let core_exists = create_read_slice(core, |machine| machine.is_on());

    let example = create_rw_signal(Example::default());

    let code = create_rw_signal("".to_owned());

    let (instruction_map, set_instruction_map) = create_signal(vec![]);

    create_effect(move |_| {
        let example = example().asm();
        code.set(example.to_owned());
    });

    view! {
        <div
            style="grid-area: pro"
            class="flex flex-col justify-center items-center content-center"
        >
            <TopBar selected_example=example/>
            <Editor code=code />
            <div class="flex w-full p-4 justify-between bg-zinc-800">
                <ResetButton/>

                {move || {
                    if core_exists() {
                        view! { <StepButton instruction_map=instruction_map/> }
                    } else {
                        view! {
                            <StartButton
                                code=code.read_only()
                                set_instruction_map=set_instruction_map
                            />
                        }
                    }
                }}

            </div>
        </div>
    }
}

#[component]
fn ResetButton() -> impl IntoView {
    let core = expect_context::<RwSignal<global_state::Machine>>();
    let highlighted_line = expect_context::<RwSignal<global_state::Highlight>>();
    let (is_started, reset) = create_slice(
        core,
        |state| state.is_on(),
        move |state, _: ()| {
            *state = global_state::Machine::Off;
            highlighted_line.set(global_state::Highlight::Off);
        },
    );

    view! {
        <button
            prop:disabled=move || !is_started()
            class="rounded-md px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm"
            class=("bg-zinc-400", move || !is_started())
            class=("text-zinc-600", move || !is_started())
            class=("bg-red-700", is_started)
            class=("hover:bg-red-600", is_started)
            on:click=move |_| reset(())
        >
            Reset
        </button>
    }
}

#[component]
fn StartButton(
    code: ReadSignal<String>,
    set_instruction_map: WriteSignal<Vec<usize>>,
) -> impl IntoView {
    let core = expect_context::<RwSignal<global_state::Machine>>();
    let vlen = expect_context::<RwSignal<Vlen>>();
    let highlighted_line = expect_context::<RwSignal<global_state::Highlight>>();
    let errors = expect_context::<RwSignal<global_state::Errors>>();

    let build_machine = create_write_slice(core, move |machine, (instructions, memory)| {
        let vu = VectorEngineBuilder::default().vlen(vlen()).build();

        let core = RvCoreBuilder::default()
            .vec_engine(vu)
            .instructions(instructions)
            .memory(memory)
            .build();

        *machine = global_state::Machine::On(core);
    });

    view! {
        <button
            class="rounded-md bg-white/10 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-white/20"
            on:click=move |_| {
                let compile_result = Interpreter::compile(code(), 0x100);
                match compile_result {
                    Err(vec) => errors.set(global_state::Errors(vec.into_iter().collect())),
                    Ok(result) => {
                        build_machine((result.instructions, result.memory));
                        let map = result.instructions_addresses;
                        if let Some(first) = map.first() {
                            highlighted_line.set(global_state::Highlight::On(*first + 1));
                        }
                        set_instruction_map(map);
                    }
                }
            }
        >

            Compile
        </button>
    }
}

#[component]
fn StepButton(instruction_map: ReadSignal<Vec<usize>>) -> impl IntoView {
    let core = expect_context::<RwSignal<global_state::Machine>>();
    let highlighted_line = expect_context::<RwSignal<global_state::Highlight>>();
    let errors = expect_context::<RwSignal<global_state::Errors>>();
    let (last_executed_line, set_last_executed_line) = create_signal(0);

    let machine_step = create_write_slice(core, move |machine, _: ()| {
        let option = machine.rw_core().unwrap().step();

        let mut did_finish = false;

        match (machine.read_core(), option) {
            // Machine executed step properly
            (Some(machine), Some(Ok(()))) => {
                let instruction_line = machine.registers.pc / 4;
                let instruction_index = instruction_map()
                    .get(instruction_line as usize)
                    .map(|el| el + 1);

                match instruction_index {
                    Some(index) => {
                        highlighted_line.set(global_state::Highlight::On(index));
                        set_last_executed_line(index);
                    }
                    None => did_finish = true,
                }
            }
            // Machine exists, but step returned error
            (Some(_), Some(Err(msg))) => {
                *machine = global_state::Machine::Off;
                errors.set(global_state::Errors(vec![(last_executed_line() - 1, msg)]));
                highlighted_line.set(global_state::Highlight::Off);
            }
            // Machine exists, but RvCore::step() returned nothing, probably impossible since
            // instruction_map().get(instruction_line) will return None first
            (Some(_), None) => did_finish = true,
            // Machine is none which should not happen in Step button, panic immediatly
            (None, _) => unreachable!(),
        };

        if did_finish {
            let finished_machine = machine.clone().finish().unwrap();
            *machine = finished_machine;
            highlighted_line.set(global_state::Highlight::Off);
        }
    });

    view! {
        <button
            class="rounded-md bg-green-700 px-3 text-sm font-semibold text-white shadow-sm hover:bg-green-600"
            on:click=move |_| machine_step(())
        >
            Step
        </button>
    }
}

use js_sys::Array;
use leptos::*;
use wasm_bindgen::{prelude::*, JsValue};

use crate::widgets::global_state;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = monacoBridge, js_name = "create")]
    fn create_monaco(parent: &JsValue);

    #[wasm_bindgen(js_namespace = monacoBridge, js_name = "onInput")]
    fn on_input(listener: &Closure<dyn Fn(String)>);

    #[wasm_bindgen(js_namespace = monacoBridge, js_name = "setInput")]
    fn set_input(input: String);

    #[wasm_bindgen(js_namespace = monacoBridge)]
    fn enable();

    #[wasm_bindgen(js_namespace = monacoBridge)]
    fn disable();

    #[wasm_bindgen(js_namespace = monacoBridge, js_name = "highlightLine")]
    fn highlight_line(line: usize);

    #[wasm_bindgen(js_namespace = monacoBridge, js_name = "setErrors")]
    fn set_errors(lines: Array, errors: Array);
}

#[component]
pub fn Editor(code: RwSignal<String>) -> impl IntoView {
    // Create

    let editor_parent = view! { <div class="h-full w-full"></div> };

    create_monaco(&editor_parent);

    // Listen to code change

    create_effect(move |_| {
        set_input(code());
    });

    let event_listener = Closure::wrap(Box::new(move |new_code: String| {
        code.set(new_code);
    }) as Box<dyn Fn(String)>);

    on_input(&event_listener);

    std::mem::forget(event_listener);

    // Update writability after compilation

    let core = expect_context::<RwSignal<global_state::Machine>>();

    create_effect(move |_| {
        use global_state::Machine;

        match core() {
            Machine::Off | Machine::Finished(_) => enable(),
            Machine::On(_) => disable(),
        }
    });

    // Set highlighted line on step

    let highlight = expect_context::<RwSignal<global_state::Highlight>>();

    create_effect(move |_| {
        use global_state::Highlight;

        match highlight() {
            Highlight::On(line) => highlight_line(line),
            Highlight::Off => highlight_line(0),
        }
    });

    // Set error markers

    let errors = expect_context::<RwSignal<global_state::Errors>>();

    create_effect(move |_| {
        let (lines, error_messages): (Vec<usize>, Vec<String>) =
            errors.get().iter().cloned().unzip();
        let js_lines = lines.iter().cloned().map(JsValue::from).collect();
        let js_errors = error_messages.iter().cloned().map(JsValue::from).collect();

        set_errors(js_lines, js_errors);
    });

    editor_parent
}

mod program_view;
mod registers_view;
mod top_bar;

use eeric_core::prelude::*;
use leptos::*;

use program_view::ProgramView;
use registers_view::RegistersView;
use top_bar::TopBar;

pub mod global_state {
    use std::ops::Deref;

    use super::*;

    #[derive(Clone, Copy)]
    pub enum Highlight {
        Off,
        On(usize),
    }

    #[derive(Clone)]
    pub struct Errors(pub Vec<(usize, String)>);

    impl Deref for Errors {
        type Target = Vec<(usize, String)>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[derive(Clone)]
    pub enum Machine {
        Off,
        On(RvCore),
        Finished(RvCore),
    }

    impl Machine {
        pub fn is_on(&self) -> bool {
            match *self {
                Self::Off | Self::Finished(_) => false,
                Self::On(_) => true,
            }
        }

        pub fn read_core(&self) -> Option<&RvCore> {
            match self {
                Self::Off => None,
                Self::On(core) => Some(core),
                Self::Finished(core) => Some(core),
            }
        }

        pub fn rw_core(&mut self) -> Option<&mut RvCore> {
            match self {
                Self::Off => None,
                Self::On(core) => Some(core),
                Self::Finished(core) => Some(core),
            }
        }

        pub fn finish(self) -> Result<Self, String> {
            match self {
                Self::Off => Err("Cannot finish off machine".to_owned()),
                Self::On(core) | Self::Finished(core) => Ok(Self::Finished(core)),
            }
        }
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, create_rw_signal(cx, global_state::Machine::Off));
    provide_context(cx, create_rw_signal(cx, Vlen::V128));
    provide_context(cx, create_rw_signal(cx, global_state::Highlight::Off));
    provide_context(cx, create_rw_signal(cx, global_state::Errors(Vec::new())));

    view! { cx,
        <div
            style=r#"
            grid-template:
            "top top" 4rem
            "pro reg" calc(100vh - 4rem) / 40vw 60vw;
            "#
            class="grid h-screen overflow-y-hidden"
        >
            <TopBar/>
            <ProgramView/>
            <RegistersView/>
        </div>
    }
}

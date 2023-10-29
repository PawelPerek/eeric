mod vector_config;
mod vector_registers;

use std::ops::Deref;

use eeric_core::prelude::*;
use leptos::*;

use vector_config::VectorConfig;
use vector_registers::VectorRegisters;

#[component]
pub fn VectorView(cx: Scope) -> impl IntoView {
    view! { cx,
        <>
            <VectorConfig/>
            <VectorRegisters/>
        </>
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct FrontEndVLEN(Vlen);

impl ToString for FrontEndVLEN {
    fn to_string(&self) -> String {
        match self.0 {
            Vlen::V64 => "64b",
            Vlen::V128 => "128b",
            Vlen::V256 => "256b",
            Vlen::V512 => "512b",
        }
        .to_string()
    }
}

impl Deref for FrontEndVLEN {
    type Target = Vlen;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FrontEndSEW {
    Default,
    Exact((BaseSew, SEWType)),
}

#[derive(Clone, Copy, PartialEq)]
pub enum SEWType {
    Int,
    Fp,
}

impl FrontEndSEW {
    fn map_default(&self, default: BaseSew) -> (BaseSew, SEWType) {
        match self {
            Self::Default => (default, SEWType::Int),
            Self::Exact(sew_pair) => *sew_pair,
        }
    }
}

impl ToString for FrontEndSEW {
    fn to_string(&self) -> String {
        match self {
            Self::Default => "Same as vector engine",
            Self::Exact((BaseSew::E8, SEWType::Int)) => "8b",
            Self::Exact((BaseSew::E16, SEWType::Int)) => "16b",
            Self::Exact((BaseSew::E32, SEWType::Int)) => "32b",
            Self::Exact((BaseSew::E64, SEWType::Int)) => "64b",
            Self::Exact((BaseSew::E8, SEWType::Fp)) => "8b (fp)",
            Self::Exact((BaseSew::E16, SEWType::Fp)) => "16b (fp)",
            Self::Exact((BaseSew::E32, SEWType::Fp)) => "32b (fp)",
            Self::Exact((BaseSew::E64, SEWType::Fp)) => "64b (fp)",
        }
        .to_owned()
    }
}

#[derive(Clone, Copy)]
pub struct FrontEndLMUL(Lmul);

impl ToString for FrontEndLMUL {
    fn to_string(&self) -> String {
        match self {
            Self(Lmul::MF8) => "1/8",
            Self(Lmul::MF4) => "1/4",
            Self(Lmul::MF2) => "1/2",
            Self(Lmul::M1) => "1",
            Self(Lmul::M2) => "2",
            Self(Lmul::M4) => "4",
            Self(Lmul::M8) => "8",
        }
        .to_owned()
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum MaskBehavior {
    #[default]
    Undisturbed,
    Agnostic,
}

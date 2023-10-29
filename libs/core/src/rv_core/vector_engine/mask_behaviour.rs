#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub enum MaskBehavior {
    #[default]
    Undisturbed,
    Agnostic,
}

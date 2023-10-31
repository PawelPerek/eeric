use super::Example;

static EXAMPLES_REPOSITORY: &[(Example, &'static str)] = &[
    (Example::Memcpy, include_str!("examples/memcpy.S")),
    (Example::Strcpy, include_str!("examples/strcpy.S")),
    (Example::Strncpy, include_str!("examples/strncpy.S")),
    (Example::Strlen, include_str!("examples/strlen.S")),
    (Example::Saxpy, include_str!("examples/saxpy.S"))
];

pub fn get_example(example: Example) -> &'static str {
    EXAMPLES_REPOSITORY
        .iter()
        .find(|&&(id, _)| id == example)
        .unwrap()
        .1
}

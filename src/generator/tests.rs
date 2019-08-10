use super::*;

#[test]
fn it_has_defaults() {
    let gen = Generator::new();
    assert_eq!(gen.word_count, 4);
    assert_eq!(gen.use_spaces, true);
}

#[test]
fn it_can_set_things() {
    let gen = Generator::new().word_count(8).use_spaces(false);
    assert_eq!(gen.word_count, 8);
    assert_eq!(gen.use_spaces, false);
}

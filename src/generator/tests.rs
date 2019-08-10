extern crate regex;
use super::*;
use regex::Regex;

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

#[test]
fn it_can_generate_passwords() {
    let has_spaces = Regex::new(r" ").unwrap();
    let has_letters = Regex::new(r"[a-zA-Z]").unwrap();
    let has_numbers = Regex::new(r"[0-9]").unwrap();

    for _x in 0..100 {
        let result = Generator::new().generate();
        assert!(result.len() > 0);
        assert!(has_spaces.is_match(&result));
        assert!(has_letters.is_match(&result));
        assert!(has_numbers.is_match(&result));
    }
}

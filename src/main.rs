extern crate rand;
use rand::Rng;

const WORDS_SIZE: usize = 60;

const WORDS: [&str; WORDS_SIZE] = [
    "common",
    "book",
    "electronic",
    "talk",
    "account",
    "mark",
    "interest",
    "written",
    "can't",
    "bed",
    "necessary",
    "age",
    "else",
    "force",
    "idea",
    "longer",
    "art",
    "spoke",
    "across",
    "brother",
    "early",
    "ought",
    "sometimes",
    "line",
    "saying",
    "table",
    "appeared",
    "river",
    "continued",
    "eye",
    "ety",
    "sun",
    "information",
    "later",
    "everything",
    "reached",
    "suddenly",
    "past",
    "hours",
    "strange",
    "deep",
    "change",
    "miles",
    "feeling",
    "act",
    "meet",
    "paid",
    "further",
    "purpose",
    "happy",
    "added",
    "seem",
    "taking",
    "blood",
    "rose",
    "south",
    "beyond",
    "cold",
    "neither",
    "forward",
];

fn main() {
    let rng = rand::thread_rng();

    let passwords: Vec<&str> = vec![
        get_random_word(rng),
        get_random_word(rng),
        get_random_word(rng),
        get_random_word(rng),
    ];

    println!(
        "{} {} {} {}",
        passwords[0], passwords[1], passwords[2], passwords[3]
    );
}

fn get_random_word<'a>(mut rng: rand::prelude::ThreadRng) -> &'a str {
    let n: usize = rng.gen_range(0, WORDS_SIZE);
    &WORDS[n]
}

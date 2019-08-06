mod words;

extern crate rand;
use rand::prelude::ThreadRng;
use rand::Rng;
use words::{WORDS, WORDS_SIZE};

fn main() {
    let rng = rand::thread_rng();
    let words = get_words(rng);

    println!("{} {} {} {}", words[0], words[1], words[2], words[3]);
}

fn get_words<'a>(rng: ThreadRng) -> Vec<&'a str> {
    vec![
        get_random_word(rng),
        get_random_word(rng),
        get_random_word(rng),
        get_random_word(rng),
    ]
}

fn get_random_word<'a>(mut rng: ThreadRng) -> &'a str {
    let n: usize = rng.gen_range(0, WORDS_SIZE);
    &WORDS[n]
}

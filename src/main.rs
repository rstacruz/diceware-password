mod words;

extern crate rand;
use rand::prelude::ThreadRng;
use rand::Rng;
use words::{WORDS, WORDS_SIZE};

fn main() {
    let gen = Generator::new();
    let passwd = gen.generate();
    println!("{}", passwd);
}

struct Generator {
    rng: ThreadRng,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            rng: rand::thread_rng(),
        }
    }

    pub fn generate(&self) -> String {
        let words = self.get_words();
        let str = format!("{} {} {} {}", words[0], words[1], words[2], words[3]);
        str
    }

    pub fn get_words<'a>(&self) -> Vec<&'a str> {
        vec![
            self.get_random_word(),
            self.get_random_word(),
            self.get_random_word(),
            self.get_random_word(),
        ]
    }

    pub fn get_random_word<'a>(&self) -> &'a str {
        let mut rng = self.rng;
        let n: usize = rng.gen_range(0, WORDS_SIZE);
        &WORDS[n]
    }
}

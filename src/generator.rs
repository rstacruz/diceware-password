extern crate rand;
use crate::words::{WORDS, WORDS_SIZE};
use rand::prelude::ThreadRng;
use rand::Rng;

pub struct Generator {
    /// Random number generator
    rng: ThreadRng,

    /// Number of words to generate
    word_count: u64,

    /// If spaces are to be used
    spaces: bool,
}

impl Generator {
    /// Create a new password generator
    ///
    /// ```
    /// let gen = Generator::new();
    /// gen.generate()
    /// // => "correct horse battery staple"
    /// ```
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            word_count: 4,
            spaces: true,
        }
    }

    pub fn word_count(mut self, n: u64) -> Self {
        self.word_count = n;
        self
    }

    pub fn spaces(mut self, value: bool) -> Self {
        self.spaces = value;
        self
    }

    /// Generate a password
    pub fn generate(&self) -> String {
        let count = self.word_count;
        let words = self.get_words(count);
        let use_spaces = self.spaces;

        if use_spaces {
            words.join(" ")
        } else {
            words.join("-")
        }
    }

    /// Returns `count` random words
    pub fn get_words<'a>(&self, count: u64) -> Vec<&'a str> {
        let mut words: Vec<&str> = vec![];
        let mut index = 0;

        while index < count {
            words.push(self.get_random_word());
            index += 1;
        }

        words
    }

    /// Returns one random word
    pub fn get_random_word<'a>(&self) -> &'a str {
        let mut rng = self.rng;
        let n: usize = rng.gen_range(0, WORDS_SIZE);
        &WORDS[n]
    }
}

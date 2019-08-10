extern crate rand;
use crate::words::{WORDS, WORDS_SIZE};
use rand::prelude::ThreadRng;
use rand::Rng;
#[cfg(test)]
mod tests;

pub struct Generator {
    /// Random number generator
    rng: ThreadRng,

    /// Number of words to generate
    word_count: u64,

    /// If spaces are to be used
    use_spaces: bool,
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
            use_spaces: true,
        }
    }

    pub fn word_count(self, value: u64) -> Self {
        Self { word_count: value, ..self }
    }

    pub fn use_spaces(self, value: bool) -> Self {
        Self { use_spaces: value, ..self }
    }

    /// Generate a password
    pub fn generate(&self) -> String {
        let count = self.word_count;
        let words = self.get_words(count);
        let use_spaces = self.use_spaces;

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

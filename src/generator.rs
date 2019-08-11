extern crate rand;
use crate::symbols::{SYMBOLS, SYMBOLS_SIZE};
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
        Self {
            word_count: value,
            ..self
        }
    }

    pub fn use_spaces(self, value: bool) -> Self {
        Self {
            use_spaces: value,
            ..self
        }
    }

    /// Generate a password
    pub fn generate(&self) -> String {
        let count = self.word_count;
        let parts = self.get_words(count);
        let parts = self.capitalize_one_word(&parts);
        let parts = self.add_junk(&parts);
        let use_spaces = self.use_spaces;

        if use_spaces {
            parts.join(" ")
        } else {
            parts.join("-")
        }
    }

    /// Adds "junk" into a vector of strings.
    ///
    /// ```
    /// self.add_junk(["hello", "world"])
    /// // => ["hello", "world", "20$"]
    /// ```
    pub fn add_junk(&self, source: &[String]) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut rng = self.rng;
        let junk = self.get_junk();

        // Find the spot to place the junk in
        let len = source.len();
        let n = rng.gen_range(1, len + 1);

        // Reconstruct result
        result.extend_from_slice(&source[0..n]);
        result.push(junk);
        result.extend_from_slice(&source[n..]);
        result
    }

    /// Gets "junk", or a string of random numbers and symbols
    /// ```
    /// self.get_junk()
    /// // => "20.1$"
    /// ```
    pub fn get_junk(&self) -> String {
        let mut rng = self.rng;

        let parts = match rng.gen_range(0, 4) {
            0 => vec![self.get_digits(), self.get_symbols()],
            1 => vec![self.get_digits(), self.get_symbols(), self.get_digits()],
            2 => vec![self.get_symbols(), self.get_digits(), self.get_symbols()],
            _ => vec![self.get_symbols(), self.get_digits()],
        };

        parts.join("")
    }

    pub fn capitalize_one_word(&self, source: &[String]) -> Vec<String> {
        let mut rng = self.rng;

        // Find the spot to place the junk in
        let len = source.len();
        let n = rng.gen_range(0, len);

        let mut result: Vec<String> = vec![];

        // Reconstruct result
        result.extend_from_slice(&source[0..n]);
        result.push(capitalize_word(&source[n]));
        result.extend_from_slice(&source[(n + 1)..]);
        result
    }

    pub fn get_symbols(&self) -> String {
        let sym = self.get_random_symbol();
        String::from(sym)
    }

    /// Returns `count` random words
    pub fn get_words(&self, count: u64) -> Vec<String> {
        let mut words: Vec<String> = vec![];
        let mut index = 0;

        while index < count {
            words.push(self.get_random_word());
            index += 1;
        }

        words
    }

    /// Returns one random number as a string.
    ///
    /// ```
    /// self.get_digits()
    /// // => "832"
    /// ```
    pub fn get_digits(&self) -> String {
        let mut rng = self.rng;
        rng.gen_range(1, 99).to_string()
    }

    /// Returns one random word.
    ///
    /// ```
    /// self.get_random_word()
    /// // => "potato"
    /// ```
    pub fn get_random_word(&self) -> String {
        let mut rng = self.rng;
        let n: usize = rng.gen_range(0, WORDS_SIZE);
        String::from(WORDS[n])
    }

    /// Returns one random symbol.
    ///
    /// ```
    /// self.get_random_symbol()
    /// // => "$"
    /// ```
    pub fn get_random_symbol(&self) -> String {
        let mut rng = self.rng;
        let n: usize = rng.gen_range(0, SYMBOLS_SIZE);
        String::from(SYMBOLS[n])
    }
}

pub fn capitalize_word(source: &str) -> String {
    let first = &source[0..1];
    let letter = first.to_ascii_uppercase();
    let rest = &source[1..];
    format!("{}{}", letter, rest)
}

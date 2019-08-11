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

/// Methods used for public consumption.
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
        let parts = self.gen_words(count);
        let parts = self.capitalize_one_word(&parts);
        let parts = self.add_junk(&parts);
        let use_spaces = self.use_spaces;

        if use_spaces {
            parts.join(" ")
        } else {
            parts.join("-")
        }
    }
}

/// These are internal delegate methods.
impl Generator {
    /// Returns `count` random words
    ///
    /// ```
    /// gen_words()
    /// // => ["hey", "yeah", "oh", "what"]
    /// ```
    pub fn gen_words(&self, count: u64) -> Vec<String> {
        let mut words: Vec<String> = vec![];
        let mut index = 0;

        while index < count {
            words.push(self.gen_random_word());
            index += 1;
        }

        words
    }

    /// Capitalizes one word in a list of words
    /// ```
    /// let source = vec!["hello", "world"]
    /// let result = capitalize_one_word(&source)
    /// // => ["hello", "World"]
    /// ```
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

    /// Adds "junk" into a vector of strings.
    /// ```
    /// self.add_junk(["hello", "world"])
    /// // => ["hello", "world", "20$"]
    /// ```
    pub fn add_junk(&self, source: &[String]) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut rng = self.rng;
        let junk = self.gen_junk();

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
    /// self.gen_junk()
    /// // => "20.1$"
    /// ```
    fn gen_junk(&self) -> String {
        let mut rng = self.rng;

        let parts = match rng.gen_range(0, 3) {
            0 => vec![self.gen_digits(), self.gen_symbols()],
            1 => vec![self.gen_digits(), self.gen_symbols(), self.gen_digits()],
            _ => vec![self.gen_symbols(), self.gen_digits()],
        };

        parts.join("")
    }

    /// Returns some symbols
    /// ```
    /// self.gen_symbols()
    /// // => "!$"
    /// ```
    pub fn gen_symbols(&self) -> String {
        let mut rng = self.rng;

        match rng.gen_range(0, 2) {
            0 => format!("{}{}", self.gen_random_symbol(), self.gen_random_symbol()),
            _ => self.gen_random_symbol(),
        }
    }

    /// Returns one random number as a string.
    ///
    /// ```
    /// self.gen_digits()
    /// // => "832"
    /// ```
    pub fn gen_digits(&self) -> String {
        let mut rng = self.rng;
        rng.gen_range(1, 99).to_string()
    }

    /// Returns one random word.
    ///
    /// ```
    /// self.gen_random_word()
    /// // => "potato"
    /// ```
    pub fn gen_random_word(&self) -> String {
        let mut rng = self.rng;
        let n: usize = rng.gen_range(0, WORDS_SIZE);
        String::from(WORDS[n])
    }

    /// Returns one random symbol.
    ///
    /// ```
    /// self.gen_random_symbol()
    /// // => "$"
    /// ```
    pub fn gen_random_symbol(&self) -> String {
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

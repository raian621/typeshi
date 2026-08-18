use std::cell::RefCell;

use rand::{RngExt, rngs::StdRng};

use crate::core::generators::traits::Generator;

/// Generates random nonsense words
pub struct Gibberish {
    min_length: usize,
    max_length: usize,
    rng: RefCell<StdRng>,
}

impl Gibberish {
    pub fn new(min_length: usize, max_length: usize) -> Self {
        Self {
            min_length,
            max_length,
            rng: RefCell::new(rand::make_rng()),
        }
    }
}

impl Default for Gibberish {
    fn default() -> Self {
        Self::new(3, 7)
    }
}

impl Generator for Gibberish {
    fn get_token(&self) -> String {
        let token_length = self
            .rng
            .borrow_mut()
            .random_range(self.min_length..=self.max_length);

        (0..token_length)
            .map(|_| self.rng.borrow_mut().random_range('a'..='z'))
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gibberish_generator() {
        let min_length = 3;
        let max_length = 7;
        let generator = Gibberish::new(min_length, max_length);

        for _ in 0..20 {
            let token = generator.get_token();
            assert!(token.len() >= min_length);
            assert!(token.len() <= max_length);
        }
    }
}

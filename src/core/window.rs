use crate::core::{
    generators::traits::Generator,
    tokens::{TokenKind, TokenizedText},
};

/// A rolling window of tokens.
pub struct Window {
    generator: Box<dyn Generator>,
    tokens: TokenizedText,
}

impl Window {
    pub fn new(generator: impl Generator + 'static) -> Self {
        Self {
            generator: Box::new(generator),
            tokens: TokenizedText::default(),
        }
    }

    /// Add tokens until the size of the window has grown by at least `length`
    /// characters.
    pub fn grow_by_length(&mut self, length: usize) {
        let mut grown_length = 0;
        while grown_length < length {
            let token = self.generator.get_token();
            grown_length += token.len() + 1;
            self.tokens.push_lexeme(token, TokenKind::Word);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::generators::gibberish::Gibberish;

    use super::*;

    #[test]
    fn test_window_grow_by_length() {
        let mut last_length = 0;
        let grow_length = 10;
        let mut window = Window::new(Gibberish::default());
        for _ in 0..20 {
            window.grow_by_length(grow_length);
            let curr_length = window
                .tokens
                .tokens
                .iter()
                .fold(window.tokens.tokens.len() - 1, |sum, token| {
                    sum + token.lexeme.len()
                });

            assert!(
                (curr_length - last_length) >= grow_length,
                "Window did not grow by {grow_length} chars"
            );
            last_length = curr_length;
        }
    }
}

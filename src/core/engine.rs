use crate::core::tokens::TokenizedText;

#[derive(Debug, Default)]
pub struct Engine {
    typed_text: TokenizedText,
}

impl Engine {
    pub fn push_char(&mut self, c: char) {
        self.typed_text.push_char(c);
    }

    pub fn pop_char(&mut self) {
        self.typed_text.pop_char();
    }

    pub fn token_diff(&self) -> String {
        // for now just output the entire typed text
        self.typed_text
            .tokens
            .iter()
            .map(|token| token.lexeme.iter().cloned().collect())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_token_diff() {
        let mut engine = Engine::default();
        "Lorem ipsum".chars().for_each(|c| engine.push_char(c));
        assert_eq!(engine.token_diff(), "Lorem ipsum");
        engine.pop_char();
        assert_eq!(engine.token_diff(), "Lorem ipsu");
    }
}

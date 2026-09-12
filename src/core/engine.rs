use crate::core::{
    generators::{gibberish::Gibberish, traits::Generator},
    tokens::{Token, TokenDiff, TokenKind, TokenizedText},
};

#[derive(Debug)]
pub struct Engine {
    typed_text: TokenizedText,
    generated_text: TokenizedText,
    _generator: Box<dyn Generator>,
}

impl Engine {
    pub fn push_char(&mut self, c: char) {
        self.typed_text.push_char(c);
    }

    pub fn pop_char(&mut self) {
        self.typed_text.pop_char();
    }

    pub fn token_diff(&self) -> Vec<TokenDiff> {
        let mut token_diffs = vec![];
        let generated_text_tokens: Vec<Token> = self
            .generated_text
            .tokens
            .iter()
            .filter(|token| token.kind != TokenKind::Whitespace)
            .cloned()
            .collect();
        let typed_text_tokens: Vec<Token> = self
            .typed_text
            .tokens
            .iter()
            .filter(|token| token.kind != TokenKind::Whitespace)
            .cloned()
            .collect();
        token_diffs.resize(generated_text_tokens.len(), Default::default());
        for i in 0..typed_text_tokens.len().min(generated_text_tokens.len()) {
            token_diffs[i] = TokenDiff::diff(&generated_text_tokens[i], &typed_text_tokens[i]);
        }
        for i in typed_text_tokens.len()..generated_text_tokens.len() {
            token_diffs[i] = TokenDiff::new(
                "",
                /* missing= */
                generated_text_tokens[i].lexeme.iter().collect::<String>(),
                "",
                TokenKind::Word,
            );
        }

        token_diffs
    }

    pub fn generate_tokens(&mut self, token_count: usize) {
        for _ in 0..token_count {
            self.generated_text
                .push_lexeme(self._generator.get_token(), TokenKind::Word);
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            typed_text: Default::default(),
            generated_text: Default::default(),
            _generator: Box::new(Gibberish::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_token_diff() {
        let mut engine = Engine::default();
        "Lorem ipsum".chars().for_each(|c| {
            engine.push_char(c);
            engine.generated_text.push_char(c);
        });
        assert_eq!(
            engine.token_diff(),
            vec![
                TokenDiff::new("Lorem", "", "", TokenKind::Word),
                TokenDiff::new("ipsum", "", "", TokenKind::Word)
            ]
        );
        engine.pop_char();
        assert_eq!(
            engine.token_diff(),
            vec![
                TokenDiff::new("Lorem", "", "", TokenKind::Word),
                TokenDiff::new("ipsu", "m", "", TokenKind::Word)
            ]
        );
    }

    #[test]
    fn test_generate_tokens() {
        let token_count = 10;
        let mut engine = Engine::default();
        engine.generate_tokens(token_count);
        assert_eq!(token_count, engine.generated_text.tokens.len());
        engine.generate_tokens(token_count);
        assert_eq!(token_count * 2, engine.generated_text.tokens.len());
    }
}

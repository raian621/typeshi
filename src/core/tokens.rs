#[derive(Clone, Debug, Default, PartialEq)]
pub struct Token {
    /// Start index of the token in the original string / text.
    pub start: usize,
    /// End index of the token in the original string / text.
    pub end: usize,
    /// Lexeme of the token.
    pub lexeme: Vec<char>,
    /// What kind of token this is.
    pub kind: TokenKind,
}

/// A fork-like diff of two tokens. For example the diff of "word" and "world"
/// would be:
/// ```
/// TokenDiff {
///   common: "wor",
///   missing: "d",
///   incorrect: "ld"
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct TokenDiff {
    /// Prefix both token's lexemes have in common.
    common: String,
    /// Suffix from the expected lexeme that doesn't match the actual lexeme.
    missing: String,
    /// Suffix from the actual lexeme that doesn't match the expected lexeme.
    incorrect: String,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum TokenKind {
    // Placeholder token kind / default value.
    #[default]
    Empty,
    /// Tab newline, or space.
    Whitespace,
    /// Anything that isn't a word or whitespace.
    Operator,
    /// A word like 'word' or 'chungus'.
    Word,
}

#[derive(Debug, Default)]
pub struct TokenizedText {
    pub tokens: Vec<Token>,
}

impl From<String> for TokenizedText {
    fn from(text: String) -> Self {
        Self::new(tokenize(text))
    }
}

impl Token {
    pub fn new(start: usize, lexeme: Vec<char>, kind: TokenKind) -> Self {
        Self {
            start,
            end: start + lexeme.len(),
            lexeme,
            kind,
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.lexeme.push(c);
        self.end += 1;
    }

    pub fn pop_char(&mut self) {
        if !self.lexeme.is_empty() {
            self.lexeme.pop();
            self.end -= 1;
        }
    }
}

impl TokenDiff {
    pub fn diff(t1: &Token, t2: &Token) -> Self {
        let size = usize::min(t1.lexeme.len(), t2.lexeme.len());
        let mut fork_idx = 0;
        while fork_idx < size && t1.lexeme[fork_idx] == t2.lexeme[fork_idx] {
            fork_idx += 1;
        }

        let common = t1.lexeme[..fork_idx].iter().collect();
        let missing = t1.lexeme[fork_idx..].iter().collect();
        let incorrect = t2.lexeme[fork_idx..].iter().collect();

        Self { common, missing, incorrect }
    }
}

impl TokenKind {
    pub fn classify(c: char) -> Self {
        if c.is_whitespace() {
            Self::Whitespace
        } else if c.is_alphanumeric() {
            Self::Word
        } else {
            Self::Operator
        }
    }
}

impl TokenizedText {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn pop_token(&mut self) {
        self.tokens.pop();
    }

    pub fn push_char(&mut self, c: char) {
        if let Some(last_token) = self.tokens.last_mut() {
            let char_token_kind = TokenKind::classify(c);
            if char_token_kind == last_token.kind {
                last_token.push_char(c);
            } else {
                let start = last_token.end;
                self.push_token(Token::new(start, vec![c], char_token_kind));
            }
        } else {
            self.push_token(Token::new(0, vec![c], TokenKind::classify(c)));
        }
    }

    pub fn pop_char(&mut self) {
        if let Some(last_token) = self.tokens.last_mut() {
            last_token.pop_char();
            if last_token.start == last_token.end {
                self.pop_token();
            }
        }
    }
}

fn tokenize(text: String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut token_buffer = Token::default();

    for c in text.chars() {
        let token_kind = TokenKind::classify(c);
        if token_kind == token_buffer.kind {
            token_buffer.push_char(c);
        } else {
            if token_buffer.kind != TokenKind::Empty {
                tokens.push(token_buffer.clone());
            }
            token_buffer.lexeme.clear();
            token_buffer.kind = token_kind;
            token_buffer.start = token_buffer.end;
            token_buffer.push_char(c);
        }
    }

    tokens.push(token_buffer);

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_default() {
        let token = Token::default();
        assert_eq!(token.kind, TokenKind::Empty);
        assert_eq!(token.start, 0);
        assert_eq!(token.end, 0);
        assert_eq!(token.lexeme.len(), 0);
    }

    #[test]
    fn test_token_push() {
        let mut token = Token::new(0, "word".chars().collect(), TokenKind::Word);
        assert_eq!(token.end, 4);
        assert_eq!(token.lexeme.iter().cloned().collect::<String>(), "word");
        token.push_char('s');
        assert_eq!(token.end, 5);
        assert_eq!(token.lexeme.iter().cloned().collect::<String>(), "words");
    }

    #[test]
    fn test_token_pop() {
        let mut token = Token::new(0, "words".chars().collect(), TokenKind::Word);
        assert_eq!(token.end, 5);
        assert_eq!(token.lexeme.iter().cloned().collect::<String>(), "words");
        token.pop_char();
        assert_eq!(token.end, 4);
        assert_eq!(token.lexeme.iter().cloned().collect::<String>(), "word");
    }

    #[test]
    fn test_tokenized_text_init() {
        let text = TokenizedText::from("Lorem ipsum.".to_owned());
        assert_eq!(text.tokens.len(), 4);
        assert_eq!(
            text.tokens,
            vec![
                Token {
                    start: 0,
                    end: 5,
                    lexeme: "Lorem".chars().collect(),
                    kind: TokenKind::Word,
                },
                Token {
                    start: 5,
                    end: 6,
                    lexeme: " ".chars().collect(),
                    kind: TokenKind::Whitespace,
                },
                Token {
                    start: 6,
                    end: 11,
                    lexeme: "ipsum".chars().collect(),
                    kind: TokenKind::Word,
                },
                Token {
                    start: 11,
                    end: 12,
                    lexeme: ".".chars().collect(),
                    kind: TokenKind::Operator,
                }
            ]
        );
    }

    #[test]
    fn test_tokenized_text_pop_char() {
        let mut text = TokenizedText::from("Lorem ipsum.".to_owned());
        text.pop_char();
        assert_eq!(
            text.tokens,
            vec![
                Token {
                    start: 0,
                    end: 5,
                    lexeme: "Lorem".chars().collect(),
                    kind: TokenKind::Word,
                },
                Token {
                    start: 5,
                    end: 6,
                    lexeme: " ".chars().collect(),
                    kind: TokenKind::Whitespace,
                },
                Token {
                    start: 6,
                    end: 11,
                    lexeme: "ipsum".chars().collect(),
                    kind: TokenKind::Word,
                },
            ]
        );
    }

    #[test]
    fn test_tokenized_text_push_char() {
        let mut text = TokenizedText::from("Lorem ipsum".to_owned());
        text.push_char('.');
        assert_eq!(
            text.tokens,
            vec![
                Token {
                    start: 0,
                    end: 5,
                    lexeme: "Lorem".chars().collect(),
                    kind: TokenKind::Word,
                },
                Token {
                    start: 5,
                    end: 6,
                    lexeme: " ".chars().collect(),
                    kind: TokenKind::Whitespace,
                },
                Token {
                    start: 6,
                    end: 11,
                    lexeme: "ipsum".chars().collect(),
                    kind: TokenKind::Word,
                },
                Token {
                    start: 11,
                    end: 12,
                    lexeme: ".".chars().collect(),
                    kind: TokenKind::Operator,
                },
            ]
        );

        // Should add a single character to the last node:
        text.push_char('.');
        assert_eq!(text.tokens.last().unwrap(), &Token {
            start: 11,
            end: 13,
            lexeme: "..".chars().collect(),
            kind: TokenKind::Operator,
        })
    }

    #[test]
    fn test_token_diff() {
        let expected_token = Token::new(0, "token".chars().collect(), TokenKind::Word);
        let actual_token = Token::new(0, "tock".chars().collect(), TokenKind::Word);

        assert_eq!(TokenDiff::diff(&expected_token, &actual_token), TokenDiff {
            common: "to".into(),
            missing: "ken".into(),
            incorrect: "ck".into(),
        });
    }
}

use crate::core::tokens::{Token, TokenKind};

/// A fork-like diff of two tokens. For example the diff of "word" and "world"
/// would be:
/// ```
/// TokenDiff {
///   common: "wor",
///   missing: "d",
///   incorrect: "ld"
/// }
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenDiff {
    /// Prefix both token's lexemes have in common.
    pub common: String,
    /// Suffix from the expected lexeme that doesn't match the actual lexeme.
    pub missing: String,
    /// Suffix from the actual lexeme that doesn't match the expected lexeme.
    pub incorrect: String,
    /// The kind of token being diffed
    pub token_kind: TokenKind,
}

impl TokenDiff {
    pub fn new(
        common: impl Into<String>,
        missing: impl Into<String>,
        incorrect: impl Into<String>,
        token_kind: TokenKind,
    ) -> Self {
        Self {
            common: common.into(),
            missing: missing.into(),
            incorrect: incorrect.into(),
            token_kind,
        }
    }

    pub fn diff(t1: &Token, t2: &Token) -> Self {
        let size = usize::min(t1.lexeme.len(), t2.lexeme.len());
        let mut fork_idx = 0;
        while fork_idx < size && t1.lexeme[fork_idx] == t2.lexeme[fork_idx] {
            fork_idx += 1;
        }

        let common = t1.lexeme[..fork_idx].iter().collect();
        let missing = t1.lexeme[fork_idx..].iter().collect();
        let incorrect = t2.lexeme[fork_idx..].iter().collect();

        Self {
            common,
            missing,
            incorrect,
            token_kind: t1.kind.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_diff() {
        let expected_token = Token::new(0, "token".chars().collect(), TokenKind::Word);
        let actual_token = Token::new(0, "tock".chars().collect(), TokenKind::Word);

        assert_eq!(
            TokenDiff::diff(&expected_token, &actual_token),
            TokenDiff {
                common: "to".into(),
                missing: "ken".into(),
                incorrect: "ck".into(),
                token_kind: TokenKind::Word
            }
        );
    }
}

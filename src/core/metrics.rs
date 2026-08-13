use crate::core::tokens::TokenDiff;

pub fn get_wpm(token_diffs: &[TokenDiff], delta_seconds: u64) -> f32 {
    let correct_words = token_diffs.iter().fold(0_usize, |count, diff| {
        count
            + if diff.missing.is_empty() && diff.incorrect.is_empty() {
                1
            } else {
                0
            }
    });

    correct_words as f32 / (delta_seconds as f32 / 60_f32)
}

pub fn get_cpm(token_diffs: &[TokenDiff], delta_seconds: u64) -> f32 {
    let correct_chars = token_diffs
        .iter()
        .fold(0, |count, diff| count + diff.common.len());

    correct_chars as f32 / (delta_seconds as f32 / 60_f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECONDS_PER_MINUTE: u64 = 60;
    const SAMPLE_TEXT: &str = "the quick brown dog jumped over the lazy fox";

    #[test]
    fn test_wpm_perfect() {
        let token_diffs = SAMPLE_TEXT
            .split(" ")
            .map(|s| TokenDiff::new(s, "", ""))
            .collect::<Vec<TokenDiff>>();
        let delta_seconds = SECONDS_PER_MINUTE;

        assert_eq!(
            get_wpm(&token_diffs, delta_seconds),
            token_diffs.len() as f32
        )
    }

    #[test]
    fn test_wpm_half_incorrect() {
        let token_diffs = SAMPLE_TEXT
            .split(" ")
            .enumerate()
            // Inject missing text into half of the token diffs:
            .map(|(i, s)| TokenDiff::new(s, if i & 1 == 0 { "missing" } else { "" }, ""))
            .collect::<Vec<TokenDiff>>();
        let delta_seconds = SECONDS_PER_MINUTE;

        assert_eq!(
            get_wpm(&token_diffs, delta_seconds),
            (token_diffs.len() / 2) as f32
        )
    }

    #[test]
    fn test_cpm_perfect() {
        let token_diffs = SAMPLE_TEXT
            .split(" ")
            .map(|s| TokenDiff::new(s, "", ""))
            .collect::<Vec<TokenDiff>>();
        let delta_seconds = SECONDS_PER_MINUTE;

        assert_eq!(
            get_cpm(&token_diffs, delta_seconds),
            // Total character count:
            token_diffs
                .iter()
                .fold(0, |count, diff| diff.common.len() + count) as f32
        )
    }

    #[test]
    fn test_cpm_half_incorrect() {
        let token_diffs = SAMPLE_TEXT
            .split(" ")
            // Add string to both common and missing to simulate a 50% error rate
            .map(|s| TokenDiff::new(s, s, ""))
            .collect::<Vec<TokenDiff>>();
        let delta_seconds = SECONDS_PER_MINUTE;

        assert_eq!(
            get_cpm(&token_diffs, delta_seconds),
            // Total character count:
            (token_diffs.iter().fold(0, |count, diff| diff.common.len()
                + diff.missing.len()
                + count)
                / 2) as f32
        );
    }
}

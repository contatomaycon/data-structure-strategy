use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

pub fn normalize_token(s: &str) -> String {
    s.nfkd()
        .filter(|c| !is_combining_mark(*c))
        .collect::<String>()
        .to_lowercase()
}

pub fn tokenize(input: &str) -> Vec<String> {
    input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| normalize_token(t))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_token_removes_diacritics() {
        assert_eq!(normalize_token("caf\u{00E9}"), "cafe");
        assert_eq!(normalize_token("A\u{00C7}UCAR"), "acucar");
    }

    #[test]
    fn tokenize_splits_and_normalizes() {
        let out = tokenize("Smartphone-Pro, C\u{00C2}mera 4K");
        assert_eq!(out, vec!["smartphone", "pro", "camera", "4k"]);
    }
}

use unicode_normalization::UnicodeNormalization;

pub fn normalize(input: String) -> String {
    let mut cleaned = String::new();

    for c in input.chars() {
        if c.is_ascii_alphanumeric() || c.is_whitespace() {
            cleaned.push(c);
        } else {
            let normalized = c.nfc().collect::<String>();
            if let Some(replacement) = normalize_accented_character(&normalized) {
                cleaned.push_str(&replacement);
            }
        }
    }

    cleaned
}

fn normalize_accented_character(input: &str) -> Option<&str> {
    match input {
        "á" | "à" | "ã" | "â" => Some("a"),
        "é" | "è" | "ê" => Some("e"),
        "í" | "ì" => Some("i"),
        "ó" | "ò" | "õ" | "ô" => Some("o"),
        "ú" | "ù" => Some("u"),
        "ç" => Some("c"),
        _ => None,
    }
}

pub fn fix_myanmar_punctuation(text: &str) -> String {
    let mut fixed = text
        .replace(',', "၊")
        .replace('.', "။")
        .replace("၊ ", "၊")
        .replace("။ ", "။");

    // Remove duplicate punctuation that AI sometimes generates
    while fixed.contains("၊၊") {
        fixed = fixed.replace("၊၊", "၊");
    }
    while fixed.contains("။။") {
        fixed = fixed.replace("။။", "။");
    }

    fixed.trim().to_string()
}

pub fn apply_line_breaks(text: &str, max_len: usize) -> String {
    if text.chars().count() <= max_len || text.contains('\n') {
        return text.to_string();
    }

    let mut result = String::new();
    let mut line_len = 0;

    // Split by spaces or punctuation
    let words: Vec<&str> = text
        .split_inclusive(|c: char| c.is_whitespace() || c == '၊' || c == '။')
        .collect();

    for word in words {
        let word_len = word.chars().count();
        if line_len + word_len > max_len && line_len > 0 {
            result.push('\n');
            line_len = 0;
        }
        result.push_str(word);
        line_len += word_len;
    }

    result.trim_end().to_string()
}

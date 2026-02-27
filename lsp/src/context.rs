use regex::Regex;
use tower_lsp::lsp_types::Position;

/// Converts an LSP Position (line + UTF-16 char offset) to a byte offset.
pub fn position_to_offset(text: &str, pos: Position) -> Option<usize> {
    let mut current_line = 0u32;
    let mut offset = 0usize;

    for line in text.split('\n') {
        if current_line == pos.line {
            let mut utf16_offset = 0u32;
            for (byte_idx, ch) in line.char_indices() {
                if utf16_offset == pos.character {
                    return Some(offset + byte_idx);
                }
                utf16_offset += ch.len_utf16() as u32;
                if utf16_offset > pos.character {
                    return None;
                }
            }
            // Cursor at end of line
            if utf16_offset == pos.character {
                return Some(offset + line.len());
            }
            return None;
        }
        offset += line.len() + 1; // +1 for '\n'
        current_line += 1;
    }
    None
}

/// Returns true if the cursor at `pos` is inside a class="..." or class='...' attribute value.
pub fn is_in_class_attribute(text: &str, pos: Position) -> bool {
    let cursor = match position_to_offset(text, pos) {
        Some(o) => o,
        None => {
            eprintln!(
                "[css-class-lsp] position_to_offset returned None for line={} char={}",
                pos.line, pos.character
            );
            return false;
        }
    };

    let before = &text[..cursor];

    // Match both class="..." (HTML/PHP) and className="..." (JSX/TSX)
    let open_re = Regex::new(r#"\bclass(?:Name)?\s*=\s*["']"#).unwrap();
    let last_match = open_re.find_iter(before).last();

    let result = if let Some(m) = last_match {
        let quote_char = before[m.end() - 1..m.end()].chars().next().unwrap();
        let inside = &before[m.end()..];
        !inside.contains(quote_char)
    } else {
        false
    };

    eprintln!("[css-class-lsp] is_in_class_attribute={result} cursor={cursor}");
    result
}

/// Returns the partial class name being typed (from last whitespace/quote to cursor).
pub fn get_word_prefix(text: &str, pos: Position) -> String {
    let cursor = match position_to_offset(text, pos) {
        Some(o) => o,
        None => return String::new(),
    };

    let before = &text[..cursor];
    let prefix_start = before
        .rfind(|c: char| c.is_whitespace() || c == '"' || c == '\'')
        .map(|i| i + 1)
        .unwrap_or(0);

    before[prefix_start..].to_string()
}

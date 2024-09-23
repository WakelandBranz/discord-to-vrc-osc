// TODO: Add functionality with VRChat API endpoints (these imports will be useful for that)
// For uploading text files
use std::fs::File;
use std::io::Write;
use reqwest;

// Thanks https://stackoverflow.com/questions/38461429/how-can-i-truncate-a-string-to-have-at-most-n-characters
pub fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub fn format_section(name: &str) -> String {
    format!("  {}:", name)
}

pub fn format_field(name: &str, value: &str) -> String {
    format!("    {}: {}", name, value)
}

pub fn format_list(name: &str, items: &[String]) -> String {
    let mut output = format!("    {}:", name);
    if items.is_empty() {
        output.push_str(" []");
    } else {
        for item in items {
            output.push_str(&format!("\n      - {}", item));
        }
    }
    output
}
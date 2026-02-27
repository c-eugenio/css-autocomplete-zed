use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use url::Url;
use walkdir::WalkDir;

const SKIP_DIRS: &[&str] = &["node_modules", ".git", ".next", "target"];

pub fn find_css_files(root: &Path) -> Vec<std::path::PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy();
                !SKIP_DIRS.contains(&name.as_ref())
            } else {
                true
            }
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            matches!(
                e.path().extension().and_then(|s| s.to_str()),
                Some("css" | "scss" | "sass" | "less")
            )
        })
        .map(|e| e.into_path())
        .collect()
}

pub fn extract_classes(css: &str) -> HashSet<String> {
    // Strip block comments
    let comment_re = Regex::new(r"/\*[\s\S]*?\*/").unwrap();
    let without_comments = comment_re.replace_all(css, " ");

    // Strip string literals (single and double quoted)
    let string_re = Regex::new(r#"(?s)("(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*')"#).unwrap();
    let without_strings = string_re.replace_all(&without_comments, " ");

    // Extract class selectors
    let class_re = Regex::new(r"\.(-?[a-zA-Z_][a-zA-Z0-9_-]*)").unwrap();
    class_re
        .captures_iter(&without_strings)
        .map(|cap| cap[1].to_string())
        .collect()
}

pub fn scan_directory(root: &Path) -> HashMap<Url, HashSet<String>> {
    let mut result = HashMap::new();
    for path in find_css_files(root) {
        if let Ok(content) = std::fs::read_to_string(&path) {
            let classes = extract_classes(&content);
            if let Ok(url) = Url::from_file_path(&path) {
                result.insert(url, classes);
            }
        }
    }
    result
}

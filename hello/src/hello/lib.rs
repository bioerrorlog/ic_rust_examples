#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

mod tests {
    use super::*;

    #[test]
    fn greeting_contains_us_string() {
        let result = greet("World".to_string());
        assert!(result.contains("World"));
    }

    #[test]
    fn greeting_contains_jp_string() {
        let result = greet("日本語の名前".to_string());
        assert!(result.contains("日本語の名前"));
    }

    #[test]
    fn greeting_contains_complex_jp_kanji_string() {
        let result = greet("竈門禰豆子".to_string());
        assert!(result.contains("竈門禰豆子"));
    }

    #[test]
    fn greeting_contains_emoji() {
        let result = greet("👪".to_string());
        assert!(result.contains("👪"));
    }
}

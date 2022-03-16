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
}

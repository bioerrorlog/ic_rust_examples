#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[test]
fn test_greeting() {
    assert_eq!(greet("World".to_string()), "Hello, World!");
}

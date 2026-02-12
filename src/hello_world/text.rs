pub async fn text() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod unit_tests {

    #[tokio::test]
    async fn should_return_text_hello_world() {
        let result = super::text().await;
        assert_eq!(result, r#"Hello, World!"#);
    }
}

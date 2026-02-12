use axum::response::Html;

pub async fn html() -> Html<String> {
    Html(format!("<h1>{}</h1>", super::text().await))
}

#[cfg(test)]
mod unit_tests {
    
    #[tokio::test]
    async fn should_return_html_hello_world() {
        let result = super::html().await;
        assert_eq!(result.0, "<h1>Hello, World!</h1>");
    }
}

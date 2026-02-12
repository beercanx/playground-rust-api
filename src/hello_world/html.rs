use axum::response::Html;

pub async fn html() -> Html<String> {
    Html(format!("<h1>{}</h1>", super::text().await))
}

#[cfg(test)]
mod unit_tests {

    use super::*;
    
    #[tokio::test]
    async fn should_return_html_hello_world() {
        let result: Html<String> = html().await;
        assert_eq!(result.0, "<h1>Hello, World!</h1>");
    }
}

use axum::Json;
use serde::Serialize;

pub async fn json() -> Json<Message> {
    Json(Message{message: super::text().await})
}

#[derive(Serialize, Eq, PartialEq, Debug)]
pub struct Message {
    message: &'static str,
}

#[cfg(test)]
mod unit_tests {

    use super::*;

    #[tokio::test]
    async fn should_return_json_hello_world() {
        let result: Json<Message> = json().await;
        assert_eq!(result.0, Message{message:"Hello, World!"});
    }
}

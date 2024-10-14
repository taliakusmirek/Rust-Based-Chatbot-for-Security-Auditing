use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: MessageResponse,
}

#[derive(Deserialize)]
pub struct MessageResponse {
    pub content: String,
}

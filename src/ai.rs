use dotenvy::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[tokio::main]
pub async fn ai_response(ques: &str) -> Result<(), Box<dyn std::error::Error>> {
    // ques.parse().unwrap();
    dotenv().ok();
     let api_key = env::var("GROQ_API_KEY")?;

    let client = reqwest::Client::new();
    let url = "https://api.groq.com/openai/v1/chat/completions";

    let body = ChatRequest {
        model: "llama3-70b-8192".to_string(), // or "llama3-70b-8192"
        messages: vec![Message {
            role: "user".to_string(),
            content: ques.parse().unwrap(),
        }],
    };

    let res = client
        .post(url)
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
        .await?;

    let text = res.text().await?;
    println!("🔍 Raw response: {}", text);
    let json: ChatResponse = serde_json::from_str(&text)?;
    println!("🤖 {}", json.choices[0].message.content.trim());

    Ok(())
}

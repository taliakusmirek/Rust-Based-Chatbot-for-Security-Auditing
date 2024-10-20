mod lib;
use lib::ThreadPool;
mod chat;
use chat::{OpenAIRequest, Message, OpenAIResponse};
use std::{
    error::Error,
    fs,
};
use dotenv::dotenv;
use std::env;
use tiny_http::{Server, Response, Header};
use serde_json::json;


fn call_gpt_api(api_key: &str, user_input: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let request_body = OpenAIRequest {
        model: "gpt-4".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: user_input.to_string(),
        }],
    };

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()?;
        
    let gpt_response: OpenAIResponse = res.json()?;
    let answer = gpt_response.choices[0].message.content.clone(); // Clone here to return a String
    Ok(answer)
}

fn handle_request(mut request: tiny_http::Request, api_key: &str) -> Result<(), Box<dyn Error>> {
    match request.url() {
        "/" => {
            let content = fs::read_to_string("index.html")?;
            let response = Response::from_string(content)
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap());
            request.respond(response)?;
        },
        "/chat" => {
            if request.method() == &tiny_http::Method::Post {
                let mut content = String::new();
                request.as_reader().read_to_string(&mut content)?;
                let json: serde_json::Value = serde_json::from_str(&content)?;
                let user_message = json["message"].as_str().unwrap_or_default();

                let gpt_response = call_gpt_api(api_key, user_message)?;
                let response_json = json!({
                    "response": gpt_response
                });

                let response = Response::from_string(response_json.to_string())
                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                request.respond(response)?;
            } else {
                let response = Response::from_string("Method Not Allowed")
                    .with_status_code(405);
                request.respond(response)?;
            }
        },
        _ => {
            let response = Response::from_string("Not Found")
                .with_status_code(404);
            request.respond(response)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let server = Server::http("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for request in server.incoming_requests() {
        let api_key_clone = api_key.clone();
        
        pool.execute(move || {
            if let Err(e) = handle_request(request, &api_key_clone) {
                eprintln!("Error handling request: {}", e);
            }
        });
    }

    Ok(())
}
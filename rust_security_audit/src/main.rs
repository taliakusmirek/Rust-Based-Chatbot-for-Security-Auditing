mod lib;
use lib::ThreadPool;
mod chat;
use reqwest::blocking::Client;
use chat::{OpenAIRequest, Message, OpenAIResponse};
use std::{
    error::Error,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn call_gpt_api(api_key: &str, user_input: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
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

fn handle_connection(mut stream: TcpStream, api_key: &String) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(&mut stream); // Keep the stream as mutable

    // Removed http_request variable

    // Check if index.html exists and return the contents, else return 404
    let (status_line, contents) = if let Ok(html_content) = fs::read_to_string("index.html") {
        ("HTTP/1.1 200 OK", html_content)
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404: File Not Found".to_string())
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

    // Loop for GPT
    loop {
        let mut user_input = String::new();

        // Read user input directly from the stream
        if reader.read_line(&mut user_input).is_err() {
            break; // Exit loop on error (e.g., client disconnected)
        }

        user_input = user_input.trim().to_string();

        if user_input.eq_ignore_ascii_case("quit") {
            break;
        }

        // Call GPT API
        let gpt_response = call_gpt_api(api_key, &user_input)
            .unwrap_or_else(|_| "Sorry, I couldn't quite understand that.".to_string());

        // Send GPT response to client
        stream.write_all(format!("Agent: {}\n", gpt_response).as_bytes())?;

    }
    Ok(())
}

fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        let api_key_clone = api_key.clone();
        
        pool.execute(move || {
            handle_connection(stream, &api_key_clone).unwrap();
        });
    }
}

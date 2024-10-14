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
use dotenv::dotenv;
use std::env;
use std::io::{ErrorKind, Write};

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

fn handle_connection(mut stream: TcpStream, api_key: &str) -> Result<(), Box<dyn Error>> {
    let (status_line, contents) = if let Ok(html_content) = fs::read_to_string("index.html") {
        ("HTTP/1.1 200 OK", html_content)
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404: File Not Found".to_string())
    };

    let response = format!("{status_line}\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);
    
    // Write the HTTP response 
    if let Err(e) = stream.write_all(response.as_bytes()) {
        if e.kind() == ErrorKind::BrokenPipe {
            return Ok(());
        } else {
            return Err(Box::new(e));
        }
    }
    stream.flush()?;  // Ensure the response is sent

    // Now create a BufReader for reading the user's input
    {
        let mut reader = BufReader::new(&mut stream);
        loop {
            let mut user_input = String::new();

            // Read the user's input
            if reader.read_line(&mut user_input).is_err() {
                break;
            }

            user_input = user_input.trim().to_string();

            if user_input.eq_ignore_ascii_case("quit") {
                break;
            }

            // Process user input (e.g., call GPT API)
            let gpt_response = call_gpt_api(api_key, &user_input)
                .unwrap_or_else(|_| "Sorry, I couldn't quite understand that.".to_string());

            // Drop the reader to release the mutable borrow on `stream`
            drop(reader);

            // Write the GPT response back to the client
            if let Err(e) = stream.write_all(format!("Agent: {}\n", gpt_response).as_bytes()) {
                if e.kind() == ErrorKind::BrokenPipe {
                    break;
                } else {
                    return Err(Box::new(e));
                }
            }
            stream.flush()?;  // Ensure the response is sent

            // Recreate the BufReader after writing
            reader = BufReader::new(&mut stream);
        }
    }

    Ok(())
}



fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        let api_key_clone = api_key.clone();
        
        pool.execute(move || {
            if let Err(e) = handle_connection(stream, &api_key_clone) {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

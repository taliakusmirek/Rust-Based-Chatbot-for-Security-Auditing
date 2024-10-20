# Rust-Based Chatbot for Security Auditing

A Rust-based web application that leverages GPT-4 to provide security audits for Rust code.

## Description

This project implements a chatbot interface that allows users to input Rust code snippets and receive security-related feedback. The backend is built with Rust, utilizing an HTTP server and thread pool implementation. The front end is a simple HTML/JavaScript interface that communicates with the Rust backend.

## Features

- Web-based chat interface
- Integration with OpenAI's GPT-4 model for code analysis
- Concurrent request handling using a custom thread pool
- Simple and lightweight HTTP server implementation

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust and Cargo installed (https://www.rust-lang.org/tools/install)
- An OpenAI API key for GPT-4 access

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/security-audit-chatbot.git
   cd security-audit-chatbot
   ```

2. Create a `.env` file in the project root and add your OpenAI API key:
   ```
   OPENAI_API_KEY=your_api_key_here
   ```

3. Build the project:
   ```
   cargo build --release
   ```

## Usage

1. Start the server:
   ```
   cargo run --release
   ```

2. Open a web browser and navigate to `http://localhost:7878`

3. Use the chat interface to input Rust code snippets and receive security-related feedback.

## Project Structure

- `src/main.rs`: Main application logic, HTTP server, and request handling
- `src/lib.rs`: Thread pool implementation
- `src/chat.rs`: Structures for OpenAI API communication
- `index.html`: Frontend chat interface

## Contributing

Contributions to this project are welcome! Please follow these steps:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Commit your changes (`git commit -m 'Add some amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- OpenAI for providing the GPT-4 API
- The Rust community for excellent documentation and crates

## Contact

If you have any questions or feedback, please open an issue on the GitHub repository.

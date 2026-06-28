use std::net::{TcpListener, TcpStream};
use std::io::{self, prelude::*};
use std::fs;

fn main() {
    let listener = create_listener("127.0.0.1:6769");
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            match stream.peer_addr() {
                Ok(addr) => println!("Connected with: {}", addr),
                Err(e) => eprintln!("Could not get peer address: {}", e),
            }
            if let Err(e) = handle_connection(&mut stream) {
                eprintln!("Error handling connection: {}", e);
            }
        }
    }
}

fn create_listener(addr: &str) -> TcpListener {
    match TcpListener::bind(addr) {
        Ok(listener) => {
            println!("Listening on port 6769");
            listener
        }
        Err(e) => {
            eprintln!("Error trying to open LISTENING socket: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_connection(stream: &mut TcpStream) -> io::Result<()> {
    read_request(stream)?;
    let html = build_response_body()?;
    send_response(stream, &html)
}

fn read_request(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let mut bucket: Vec<u8> = Vec::new();

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Err(io::Error::new(io::ErrorKind::ConnectionAborted, "client disconnected"));
        }
        let data = String::from_utf8_lossy(&buffer[..bytes_read]);
        bucket.extend(&buffer[..bytes_read]);

        if bucket.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }

        println!("{}", data);
    }

    Ok(())
}

fn send_response(stream: &mut TcpStream, body: &str) -> io::Result<()> {
    stream.write_all(b"HTTP/1.1 200 OK\r\n")?;
    stream.write_all(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes())?;
    stream.write_all(body.as_bytes())
}

fn build_response_body() -> io::Result<String> {
    let conteudo = fs::read_to_string("index.html")?;
    Ok(inject_into_html(&conteudo, "</body>", "<p>Olá, bem vindo!</p>\n"))
}

fn inject_into_html(html: &str, target: &str, injection: &str) -> String {
    match html.find(target) {
        Some(pos) => {
            let mut result = String::with_capacity(html.len() + injection.len());
            result.push_str(&html[..pos]);
            result.push_str(injection);
            result.push_str(&html[pos..]);
            result
        }
        None => html.to_string(),
    }
}

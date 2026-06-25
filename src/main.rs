#[allow(unused)]
use std::net::{TcpListener};
use std::io::prelude::*;


fn main(){
    let listener = match TcpListener::bind("127.0.0.1:6769"){
        Ok(value) => {
                                    println!("Listening on port 6769");
                                    value
        },
        Err(_) => {
                    println!("Error trying to open LISTENING socket"); 
                    return 
                },
    };
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            println!("Connected with: {}",stream.peer_addr().unwrap());

            let mut buffer = [0;1024];
            loop {
                let bytes_read  = stream.read(&mut buffer).unwrap();
                if bytes_read == 0 {break;}
                let data = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("{}",data);
            }
            println!("Connection ended by client.")
        }
    }   
}

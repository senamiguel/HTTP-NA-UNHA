use std::net::{TcpListener};
#[allow(unused)]

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:6769"){
        Ok(value) => {
                                    println!("Listening on port 6769");
                                    value
        },
        Err(_) => {
                    println!("Error trying to open LISTENING socket"); 
                    return},
    };
    for stream in listener.incoming() {
        println!("Recebinha da silva!")
    }   
    
}

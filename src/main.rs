use std::{net::TcpListener, io::Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Conection established : {:?}", stream);

        stream.shutdown(std::net::Shutdown::Both);
    }
}

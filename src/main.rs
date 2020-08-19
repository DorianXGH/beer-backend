use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use stellarium_control_protocol::Connection;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let con = Connection::from_stream(&mut stream)
    con.readmsg().unwrap();
}

mod stellarium_control_protocol;

use std::net::TcpListener;
use std::net::TcpStream;
use stellarium_control_protocol::Connection;
use stellarium_control_protocol::GotoMsg;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut con = Connection::from_stream(&mut stream);
    while(true)
    {
        match(con.readmsg())
        {
            Some(a) => println!("Msg received : {}", a.declination),
            None => (),
        }
    }
    
}

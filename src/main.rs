mod stellarium_control_protocol;
mod convert;
mod beer_sp;

use std::net::TcpListener;
use std::net::TcpStream;
use stellarium_control_protocol::Connection;
use stellarium_control_protocol::GotoMsg;
use astro::coords::EqPoint;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut con = Connection::from_stream(&mut stream);
    loop
    {
        match con.readmsg()
        {
            Some(a) => {
                let (alt,az) = get_altaz_coord(a);
                println!("Msg received : {} {}", alt, az);

            },
            None => (),
        }
    }
    
}

fn get_altaz_coord(msg : GotoMsg) -> (f64,f64)
{
    let eq = EqPoint {
        dec : (std::f64::consts::PI/2.0) * (msg.declination as f64)/((0x4000_0000 as u64) as f64),
        asc : (2.0*std::f64::consts::PI) * (msg.right_ascension as f64)/((0x1_0000_0000 as u64) as f64)
    };
    println!("{} {}", eq.dec, eq.asc);
    convert::eq_to_altaz(eq, 0.795, 0.103)
}

// use std::io;
use std::net::TcpListener;
use std::error::Error;
use std::io::{Read, Write};
// use std::net::TcpStream;
use std::str;

pub mod init_config;

fn main() -> Result<(), Box<dyn Error>>{
    let config = init_config::Config::new(init_config::init_config());

    let listener = TcpListener::bind(config.addr).expect("Address or port cannot use!");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");

        let mut buffer = [0; 5];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();

        println!("Response from client: {:#?}", str::from_utf8(&buffer));
    }
    Ok(())
}


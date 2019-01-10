extern crate chrono;
#[macro_use]
extern crate clap;

use clap::App;
use std::net::{TcpListener, TcpStream, SocketAddr, Shutdown};
use std::io::prelude::*;
use chrono::prelude::*;
use std::thread;
use std::process::Command;

fn main() {
    let yml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yml).get_matches();
    // SocketAddr expects an unsigned int
    let mut port: u16 = 3005;

    // allow setting the port via command line -p/--port
    if let Some(p) = matches.value_of("port") {
        port = p.parse::<u16>().unwrap();
    }

    println!("> Listening on port {}", port);

    let uri = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(&uri).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    respond_to(stream);
                });
            }
            Err(_e) => {
                println!("Conn error");
            }
        }
    }

    drop(listener);
}

fn respond_to(mut stream: TcpStream) {
    let dt = Local::now();
    let message = format!("{}/{}/{} - {}", dt.month(), dt.day(), dt.year(), stream.peer_addr().unwrap());
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.write(b"Sup").unwrap();
    stream.flush().unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}

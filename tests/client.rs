extern crate lircd;
extern crate rand;

use std::net::TcpStream;
use std::thread;
use rand::{thread_rng, Rng};

use lircd::net;

const DEFAULT_LISTEN_ADDR : &'static str = "127.0.0.1";

fn init_serv() -> (&'static str, u16)
{
    let port: u16 = thread_rng().gen_range(1024, 65535);
    let listen_addr = format!("127.0.0.1:{}", port);
    thread::spawn(move || {
        net::run(&listen_addr);
    });

    thread::sleep_ms(1000);

    (DEFAULT_LISTEN_ADDR, port)
}

#[test]
fn can_connect()
{
    let mut socket = TcpStream::connect(init_serv()).unwrap();
}

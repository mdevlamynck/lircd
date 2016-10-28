extern crate lircd;
extern crate rand;

use std::io::{Read, Write, BufRead, BufReader};
use std::net::TcpStream;
use std::thread;
use rand::{thread_rng, Rng};
use std::str;

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

#[test]
fn round_echo_server()
{
    let server = init_serv();
    let mut client1 = TcpStream::connect(server).unwrap();
    let mut client2 = TcpStream::connect(server).unwrap();

    let mut client1_reader = BufReader::new(client1.try_clone().unwrap());
    let mut client2_reader = BufReader::new(client2.try_clone().unwrap());

    client1.write(b"Hi!\n");
    client1.flush();

    let mut client1_buffer = String::new();
    let mut client2_buffer = String::new();

    client1_reader.read_line(&mut client1_buffer);
    client2_reader.read_line(&mut client2_buffer);

    assert_eq!("Hi!\n", &client1_buffer);
    assert_eq!("Hi!\n", &client2_buffer);

    client2.write(b"It works!\n");
    client2.flush();

    client1_buffer.clear();
    client2_buffer.clear();

    client1_reader.read_line(&mut client1_buffer);
    client2_reader.read_line(&mut client2_buffer);

    assert_eq!("It works!\n", &client1_buffer);
    assert_eq!("It works!\n", &client2_buffer);
}

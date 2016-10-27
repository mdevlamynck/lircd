extern crate mioco;

use std::net::SocketAddr;
use std::str::FromStr;
use std::io::{self, Read, Write, BufRead, BufReader};
use self::mioco::tcp::TcpListener;

pub const DEFAULT_LISTEN_ADDR : &'static str = "0.0.0.0:6667";

pub fn run(listen_addr: &str)
{
    let listen_addr = listen_addr.to_string();

    mioco::start(move || -> io::Result<()> {
        let addr = FromStr::from_str(&listen_addr).unwrap();

        let listener = try!(TcpListener::bind(&addr));

        loop {
            let mut conn = try!(listener.accept());

            mioco::spawn(move || -> io::Result<()> {
                Ok(())
            });
        }
    }).unwrap().unwrap();
}

#[cfg(test)]
mod test
{
}

extern crate mioco;

use std::str::FromStr;
use std::io::{self, Write, BufRead, BufReader};
use self::mioco::tcp::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use irc::{Irc, Client};

pub const DEFAULT_LISTEN_ADDR : &'static str = "0.0.0.0:6667";

pub fn run(listen_addr: &str)
{
    let listen_addr = listen_addr.to_string();

    mioco::start(move || -> io::Result<()> {
        let listen_addr = FromStr::from_str(&listen_addr).unwrap_or_else(|e| {
            error!("{}, is not a valid address, using default {} instead", listen_addr, DEFAULT_LISTEN_ADDR);
            error!("original error: {}", e);
            FromStr::from_str(DEFAULT_LISTEN_ADDR).unwrap()
        });

        let listener = try!(TcpListener::bind(&listen_addr));

        let global_state = Arc::new(Mutex::new(Irc::<TcpStream>::new()));

        loop {
            let thread_state = global_state.clone();
            let conn = try!(listener.accept());

            let buf_read = BufReader::new(conn.try_clone().unwrap());

            {
                let mut state = thread_state.lock().unwrap();
                state.users.push(Client::new(conn));
            }

            mioco::spawn(move || -> io::Result<()> {
                for line in buf_read.lines() {
                    let mut state = thread_state.lock().unwrap();
                    let message = line.unwrap();

                    for mut user in state.users.iter_mut() {
                        try!(user.socket.write(message.as_bytes()));
                        try!(user.socket.write(b"\n"));
                        try!(user.socket.flush());
                    }
                }

                Ok(())
            });
        }
    }).unwrap().unwrap();
}

#[cfg(test)]
mod test
{
    //#[test]
    //fn test_graciously_ignore_invalid_listen_addr()
    //{
    //    super::run("aunris");
    //}
}

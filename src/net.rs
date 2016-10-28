extern crate mioco;

use std::net::SocketAddr;
use std::str::FromStr;
use std::io::{self, Read, Write, BufRead, BufReader};
use self::mioco::tcp::{TcpListener, TcpStream};

use std::sync::{Arc, Mutex};

pub const DEFAULT_LISTEN_ADDR : &'static str = "0.0.0.0:6667";

struct Irc<T>
    where T: Write
{
    pub users: Vec<User<T>>,
}

impl<T> Irc<T>
    where T: Write
{
    pub fn new() -> Irc<T>
    {
        Irc::<T> {
            users: Vec::new()
        }
    }
}

struct User<T>
    where T: Write
{
    pub socket: T,
}

impl<T> User<T>
    where T: Write
{
    pub fn new(socket: T) -> User<T>
    {
        User::<T> {
            socket: socket,
        }
    }
}

pub fn run(listen_addr: &str)
{
    let listen_addr = listen_addr.to_string();

    mioco::start(move || -> io::Result<()> {
        let addr = FromStr::from_str(&listen_addr).unwrap();
        let listener = try!(TcpListener::bind(&addr));

        let global_state = Arc::new(Mutex::new(Irc::<TcpStream>::new()));

        loop {
            let thread_state = global_state.clone();
            let mut conn = try!(listener.accept());

            let mut buf_read = BufReader::new(conn.try_clone().unwrap());

            {
                let mut state = thread_state.lock().unwrap();
                state.users.push(User::new(conn));
            }

            mioco::spawn(move || -> io::Result<()> {
                for line in buf_read.lines() {
                    let mut state = thread_state.lock().unwrap();
                    let message = line.unwrap();

                    for mut user in state.users.iter_mut() {
                        user.socket.write(message.as_bytes());
                        user.socket.write(b"\n");
                        user.socket.flush();
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
}

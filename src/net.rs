extern crate mioco;

use std::io::{self, Write, BufRead, BufReader};
use self::mioco::tcp::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use irc::{Irc, Client};
use config::Config;

fn root_mioco_routine(config: Config) -> io::Result<()>
{
    let listen_addr  = config.listen_addr.parse().expect("Unable to parse socket address");

    let listener     = try!(TcpListener::bind(&listen_addr));
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
}
pub fn run(config: Config)
{
    mioco::start(move || -> io::Result<()> {
        root_mioco_routine(config)
    });
}

#[cfg(test)]
mod test
{
    extern crate mioco;

    use config::Config;
    use std::io;
    use std::error::Error;

    #[test]
    fn test_raise_error_on_invalid_listen_addr()
    {
        let mut config = Config::default();
        config.listen_addr = "definitely not a network address".to_string();

        let result = mioco::spawn(move || -> io::Result<()> {
            super::root_mioco_routine(config)
        }).join();

        assert!(result.is_err());
    }
}

extern crate mioco;
extern crate memchr;
extern crate simple_signal;

use std::io::Write;
use self::mioco::tcp::{TcpListener, TcpStream};
use self::simple_signal::{Signals, Signal};
use std::sync::{Arc, Mutex};
use irc::{Irc, Client};
use config::Config;
use errors::NetResult;
use reader::{MaxLengthedBufRead, MaxLengthedBufReader};

pub fn run(config: Config)
{
    let (shutdown_tx, shutdown_rx) = mioco::sync::mpsc::channel();

    let join_handle = mioco::spawn(move || -> NetResult {
        let _ = mioco::spawn(move || {
            let _ = shutdown_rx.recv();
            mioco::shutdown();
        });

        root_mioco_routine(config)
    });

    Signals::set_handler(&[Signal::Term, Signal::Int], move |signals| {
        info!("Recieved signal {:?}, stopping...", signals);
        shutdown_tx.send(()).unwrap();
    });

    let result = join_handle.join();

    match result {
        Ok(inner_result) => {
            match inner_result {
                Ok(_)    => info!("Terminated successfully"),
                Err(err) => error!("Error occured: {}", err),
            };
        },
        Err(_) => info!("Stopped by signal"),
    }
}

fn root_mioco_routine(config: Config) -> NetResult
{
    let listen_addr  = try!(config.listen_addr.parse());
    let listener     = try!(TcpListener::bind(&listen_addr));

    let global_state = Arc::new(Mutex::new(Irc::<TcpStream>::new()));

    loop {
        let socket        = try!(listener.accept());
        let socket_reader = MaxLengthedBufReader::new(try!(socket.try_clone()));

        {
            let mut state = global_state.lock().unwrap();
            state.users.push(Client::new(socket));
        }

        let routine_state = global_state.clone();

        mioco::spawn(move || -> NetResult {
            for line in socket_reader.lines_without_too_long() {
                let mut state = routine_state.lock().unwrap();
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

#[cfg(test)]
mod test
{
    extern crate mioco;
    extern crate memchr;

    use config::Config;
    use errors::NetResult;
    use std::error::Error;

    #[test]
    fn raise_error_on_invalid_listen_address()
    {
        let _ = mioco::start(|| {
            let mut config = Config::default();
            config.listen_addr = "definitely not a network address".to_string();

            let result = mioco::spawn(move || -> NetResult {
                super::root_mioco_routine(config)
            }).join().ok().unwrap();

            assert!(result.is_err());
            assert_eq!("invalid IP address syntax", result.err().unwrap().description());

            mioco::shutdown();
        });
    }

    #[test]
    fn raise_error_on_fail_to_bind_address()
    {
        let _ = mioco::start(|| {
            let mut config = Config::default();
            config.listen_addr = "127.0.0.1:1".to_string();

            let result = mioco::spawn(move || -> NetResult {
                super::root_mioco_routine(config)
            }).join().ok().unwrap();

            assert!(result.is_err());
            assert_eq!("permission denied", result.err().unwrap().description());

            mioco::shutdown();
        });
    }
}

extern crate mioco;
extern crate simple_signal;

use std;
use std::io::Read;
use self::mioco::tcp::{TcpListener, TcpStream};
use self::mioco::unix::{UnixListener, UnixStream};
use self::simple_signal::{Signals, Signal};
use config::Config;
use errors::NetResult;
use std::path::Path;
use common_api::{Listen, Stream, Spawn, Async, Blocking};

pub trait StatefullProtocol
{
    type O;
    type H: StatefullHandle;

    fn new_connection(&self, output: Self::O) -> Self::H;
}

pub trait StatefullHandle
{
    fn consume<I: Read>(self, input: I) -> NetResult;

    fn handle_request(&self, request: String) -> NetResult;
}

pub fn run<P>(config: Config, protocol: P)
    where P: StatefullProtocol<O = TcpStream> + Send + 'static,
          P::H: Send + 'static
{
    let (shutdown_tx, shutdown_rx) = mioco::sync::mpsc::channel();

    let join_handle = mioco::spawn(move || -> NetResult {
        let _ = mioco::spawn(move || {
            let _ = shutdown_rx.recv();
            mioco::shutdown();
        });

        if config.use_async {
            try!(listen::<mioco::tcp::TcpListener, Async, P>(protocol, &config.listen_addr));
        }

        Ok(())
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

fn listen<L, S, P>(protocol: P, address: &str) -> NetResult
    where L: Listen,
          S: Spawn<NetResult>,
          P: StatefullProtocol<O = L::Stream>,
          P::H: Send + 'static
{
    let listener = try!(L::bind(address));

    loop {
        let input_socket  = try!(listener.accept());
        let output_socket = try!(input_socket.try_clone());

        let handle        = protocol.new_connection(output_socket);

        S::spawn(move || -> NetResult {
            try!(handle.consume(input_socket));

            Ok(())
        });
    }
}

#[cfg(test)]
mod test
{
    extern crate mioco;

    use config::Config;
    use errors::NetResult;
    use std::error::Error;
    use self::mioco::tcp::TcpStream;
    use super::{StatefullProtocol, StatefullHandle};

    struct DummyProtocol;
    struct DummyHandle;

    impl StatefullProtocol for DummyProtocol
    {
        type O = TcpStream;
        type H = DummyHandle;

        fn new_connection(&self, output: Self::O) -> Self::H
        {
            DummyHandle {}
        }
    }

    impl StatefullHandle for DummyHandle
    {
        fn consume<I = TcpStream>(self, input: I) -> NetResult
        {
            Ok(())
        }

        fn handle_request(&self, request: String) -> NetResult
        {
            Ok(())
        }
    }

    #[test]
    fn raise_error_on_invalid_listen_address()
    {
        let _ = mioco::start(|| {
            let mut config = Config::default();
            config.listen_addr = "definitely not a network address".to_string();

            let result = mioco::spawn(move || -> NetResult {
                
                super::tcp_listener(config, DummyProtocol {})
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
                super::tcp_listener(config, DummyProtocol {})
            }).join().ok().unwrap();

            assert!(result.is_err());
            assert_eq!("permission denied", result.err().unwrap().description());

            mioco::shutdown();
        });
    }

    #[test]
    fn calls_new_connection()
    {
        // TODO
    }

    #[test]
    fn calls_consume()
    {
        // TODO
    }
}

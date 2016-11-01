extern crate mioco;
extern crate memchr;
extern crate simple_signal;

use self::mioco::tcp::{TcpListener, TcpStream};
use self::mioco::unix::{UnixListener, UnixStream};
use self::simple_signal::{Signals, Signal};
use config::Config;
use errors::NetResult;
use std::path::Path;

pub trait StatefullProtocol
{
    type I;
    type O;
    type H: StatefullHandle;

    fn new_connection(&self, input: Self::I, output: Self::O) -> Self::H;

    fn handle_request(&self, request: String) -> NetResult;
}

pub trait StatefullHandle
{
    fn consume(self) -> NetResult;
}

pub fn run<P>(config: Config, protocol: P)
    where P: StatefullProtocol<I = TcpStream, O = TcpStream> + Send + 'static,
          P::H: Send + 'static
{
    let (shutdown_tx, shutdown_rx) = mioco::sync::mpsc::channel();

    let join_handle = mioco::spawn(move || -> NetResult {
        let _ = mioco::spawn(move || {
            let _ = shutdown_rx.recv();
            mioco::shutdown();
        });

        tcp_listener(config, protocol)
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

fn tcp_listener<P>(config: Config, protocol: P) -> NetResult
    where P: StatefullProtocol<I = TcpStream, O = TcpStream>,
          P::H: Send + 'static
{
    let listen_addr  = try!(config.listen_addr.parse());
    let listener     = try!(TcpListener::bind(&listen_addr));

    loop {
        let input_socket  = try!(listener.accept());
        let output_socket = try!(input_socket.try_clone());

        let handle    = protocol.new_connection(input_socket, output_socket);

        mioco::spawn(move || -> NetResult {
            try!(handle.consume());

            Ok(())
        });
    }
}

fn unix_listener<P>(config: Config, protocol: P) -> NetResult
    where P: StatefullProtocol<I = UnixStream, O = UnixStream>,
          P::H: Send + 'static
{
    let listen_addr  = Path::new(&config.listen_addr);
    let listener     = try!(UnixListener::bind(&listen_addr));

    loop {
        let input_socket  = try!(listener.accept());
        let output_socket = try!(input_socket.try_clone());

        let handle    = protocol.new_connection(input_socket, output_socket);

        mioco::spawn(move || -> NetResult {
            try!(handle.consume());

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
        type I = TcpStream;
        type O = TcpStream;
        type H = DummyHandle;

        fn new_connection(&self, input: Self::I, output: Self::O) -> Self::H
        {
            DummyHandle {}
        }

        fn handle_request(&self, request: String) -> NetResult
        {
            Ok(())
        }
    }

    impl StatefullHandle for DummyHandle
    {
        fn consume(self) -> NetResult
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

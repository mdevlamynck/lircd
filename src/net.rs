extern crate mioco;
extern crate simple_signal;

use std;
use std::io::{Read, Write};
use self::simple_signal::{Signals, Signal};
use irc::IrcProtocol;
use config::Config;
use error::NetResult;
use common_api::{Listen, Stream, Spawn, Async, Blocking};

pub trait StatefullProtocol<Output>
    where Output: Write
{
    type Handle: StatefullHandle<Output>;

    fn new(config: Config) -> Self;

    fn new_connection(&self, output: Output) -> Self::Handle;
}

pub trait StatefullHandle<Output>
    where Output: Write
{
    fn consume<Input: Read>(self, input: Input) -> NetResult;
}

pub fn run(config: Config)
{
    if config.use_async {
        let (shutdown_tx, shutdown_rx) = mioco::sync::mpsc::channel();

        let join_handle = mioco::spawn(move || -> NetResult {
            let _ = mioco::spawn(move || {
                let _ = shutdown_rx.recv();
                mioco::shutdown();
            });

            if config.is_unix {
                listen::<mioco::unix::UnixListener, Async>(config)?;
            } else {
                listen::<mioco::tcp::TcpListener, Async>(config)?;
            }

            Ok(())
        });

        Signals::set_handler(&[Signal::Term, Signal::Int], move |signals| {
            info!("Recieved signal {:?}, stopping...", signals);
            shutdown_tx.send(()).unwrap();
        });

        match join_handle.join() {
            Ok(inner_result) => {
                match inner_result {
                    Ok(_)    => info!("Terminated successfully"),
                    Err(err) => error!("Error occured: {}", err),
                };
            },
            Err(_) => info!("Stopped by signal"),
        }
    } else {
        Blocking::spawn(move || -> NetResult {
            if config.is_unix {
                listen::<std::os::unix::net::UnixListener, Blocking>(config)?;
            } else {
                listen::<std::net::TcpListener, Blocking>(config)?;
            }

            Ok(())
        });
    }
}

fn listen<L, S>(config: Config) -> NetResult
    where L: Listen,
          S: Spawn<NetResult>
{
    let listener = L::bind(&config.listen_address)?;
    let protocol = IrcProtocol::<L::Stream>::new(config);

    loop {
        let input_socket  = listener.accept()?;
        let output_socket = input_socket.try_clone()?;

        let handle        = protocol.new_connection(output_socket);

        S::spawn(move || -> NetResult {
            handle.consume(input_socket)?;

            Ok(())
        });
    }
}

#[cfg(test)]
mod test
{
    #[test]
    fn raise_error_on_invalid_listen_address()
    {
        // TODO
    }

    #[test]
    fn raise_error_on_fail_to_bind_address()
    {
        // TODO
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

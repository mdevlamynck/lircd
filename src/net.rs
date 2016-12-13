// IRC server in Rust
// Copyright (C) 2016, Matthias Devlamynck
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
// 
// For any questions, feture request or bug reports please contact me
// at matthias.devlamynck@mailoo.org. The official repository for this
// project is https://github.com/mdevlamynck/lircd.

extern crate mioco;

use std;
use std::io::{Read, Write};
use simple_signal::{self, Signal};
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
    if config.inner.network.use_async {
        let (shutdown_tx, shutdown_rx) = mioco::sync::mpsc::channel();

        let join_handle = mioco::spawn(move || -> NetResult {
            let _ = mioco::spawn(move || {
                let _ = shutdown_rx.recv();
                mioco::shutdown();
            });

            listen::<mioco::tcp::TcpListener, Async>(config)
        });

        simple_signal::set_handler(&[Signal::Term, Signal::Int], move |signals| {
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
        let _ = listen::<std::net::TcpListener, Blocking>(config);
    }
}

fn listen<L, S>(config: Config) -> NetResult
    where L: Listen,
          S: Spawn<NetResult>
{
    let listener = L::bind(&config.inner.network.listen_address)?;
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

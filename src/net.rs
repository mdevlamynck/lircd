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

use std::io::{Read, Write};
use error::NetResult;
use config;
use futures::{Future, Stream, Sink, IntoFuture};
use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpListener;
use tokio_core::io::Io;
use tokio_core::io::{Codec, EasyBuf};
use std::str::FromStr;
use std::net::SocketAddr;
use std::io;
use std::str;
use memchr;

pub trait StatefullProtocol<Output>
    where Output: Write
{
    type Handle: StatefullHandle<Output>;

    fn new() -> Self;

    fn new_connection(&self, output: Output) -> Self::Handle;
}

pub trait StatefullHandle<Output>
    where Output: Write
{
    fn consume<Input: Read>(self, input: Input) -> NetResult;
}

pub struct LineCodec;

impl Codec for LineCodec 
{
    type In  = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>>
    {
        if let Some(i) = memchr::memchr(b'\n', buf.as_slice()) {
            let line = buf.drain_to(i);

            buf.drain_to(1);

            match str::from_utf8(line.as_slice()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: String, buf: &mut Vec<u8>) -> io::Result<()>
    {
        buf.extend(msg.as_bytes());
        buf.push(b'\n');
        Ok(())
    }
}

pub fn run()
{
    let mut core = Core::new()
        .expect("Can't create reactor, shouldn't happen!");

    let handle = core.handle();
    let _      = core.run(main(&handle));
}

fn main<'a>(handle: &'a Handle) -> impl Future<Item=(), Error=()> + 'a
{
    let tcp    = tcp_listener(handle);

    // This lines crashes the compiler at the moment
    //let config = config_reload(handle);
    //quit(handle).select(tcp.join(config).map(|_| {}).map_err(|_| {}))

    quit(handle).select(tcp)
        .then(|_| Ok(()))
}

fn config_reload<'a>(handle: &'a Handle) -> impl Future<Item=(), Error=()> + 'a
{
    signal::reload(handle)
        .for_each(move |_| {
            info!("Reloading configuration");
            config::reload();
            Ok(())
        })
        .then(|_| Ok(()))
}

fn quit<'a>(handle: &'a Handle) -> impl Future<Item=(), Error=()> + 'a
{
    signal::quit(handle)
        .into_future()
        .then(|_| {
            info!("Shutting down");
            Ok(())
        })
}

fn tcp_listener<'a>(handle: &'a Handle) -> impl Future<Item=(), Error=()> + 'a
{
    let config = config::get().read().unwrap();
    let addr   = SocketAddr::from_str(&config.inner.network.listen_address).unwrap();
    info!("Listening on {}", addr);

    TcpListener::bind(&addr, &handle)
        .into_future()
        .and_then(move |tcp_listener| {
            tcp_listener.incoming()
                .for_each(move |(socket, _peer_addr)| {
                    let (writer, reader) = socket.framed(LineCodec).split();
                    let responses        = reader.and_then(|req| Ok(req));
                    let server           = writer.send_all(responses).then(|_| Ok(()));

                    handle.spawn(server);
                    Ok(())
                })
        })
        .map_err(|err| error!("Can't bind tcp socket : {}", err))
        .then(|_| Ok(()))
}

mod signal {
    use futures::{Future, Stream};
    use tokio_core::reactor::Handle;
    use tokio_signal::unix::{Signal, SIGHUP, SIGINT, SIGTERM};

    pub fn reload(handle: &Handle) -> impl Stream
    {
        Signal::new(SIGHUP, handle).wait()
            .expect("Can't setup SIGHUP signal listener")
            .map(|_| info!("Received SIGHUP"))
    }

    pub fn quit(handle: &Handle) -> impl Stream
    {
        let int  = Signal::new(SIGINT, handle).wait()
            .expect("Can't setup SIGINT signal listener")
            .map(|_| info!("Received SIGINT"));

        let term = Signal::new(SIGTERM, handle).wait()
            .expect("Can't setup SIGTERM signal listener")
            .map(|_| info!("Received SIGTERM"));

        int.merge(term)
    }
}

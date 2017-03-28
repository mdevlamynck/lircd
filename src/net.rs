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
use bytes::{BytesMut};
use tokio_io::AsyncRead;
use tokio_io::codec::{Encoder, Decoder};
use std::str::FromStr;
use std::net::SocketAddr;
use std::io;
use std::str;
use std::cmp::min;
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

const MAX_LINE_LENGTH: usize = 512 * 4;

pub struct LineCodec;

impl Decoder for LineCodec
{
    type Item  = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>>
    {
        let make_string = |buf: &mut BytesMut, i: usize| -> io::Result<Option<Self::Item>> {
            let line = buf.split_to(i);

            buf.split_to(1);

            match str::from_utf8(&line) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid UTF-8")),
            }
        };

        let drop_input = |buf: &mut BytesMut, i: usize| -> io::Result<Option<Self::Item>> {
            buf.split_to(i);
            Ok(None)
        };

        let len = buf.len();

        match memchr::memchr(b'\n', &buf) {
            Some(i) if i < MAX_LINE_LENGTH => make_string(buf, i),
            Some(i)                        => drop_input(buf, i),
            _                              => drop_input(buf, min(len, MAX_LINE_LENGTH)),
        }
    }
}

impl Encoder for LineCodec
{
    type Item  = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()>
    {
        buf.extend(msg.as_bytes());
        buf.extend([b'\n'].iter());
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

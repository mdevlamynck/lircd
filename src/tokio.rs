extern crate futures;
extern crate tokio_core;
extern crate tokio_signal;
extern crate libc;

use self::futures::{Future, Stream};
use self::tokio_core::reactor::{Core, Handle};
use self::tokio_core::net::TcpListener;
use self::tokio_signal::unix::{Signal, SIGHUP, SIGINT, SIGTERM};
use self::libc::c_int;
use std::str::FromStr;
use std::net::SocketAddr;

pub fn run()
{
    let mut core = Core::new().expect("Can't create reactor, shouldn't happen!");
    let handle = core.handle();

    core.run(main(&handle));
}

fn main(handle: &Handle) -> impl Future
{
    let quit_signal  = quit_signal(handle).into_future().map(|_| {}).map_err(|_| {});
    let tcp_listener = tcp_listener(handle).map(|_| {}).map_err(|_| {});

    quit_signal.select(tcp_listener)
    //quit_signal
}

fn reload_signal(handle: &Handle) -> impl Stream
{
    let hup: Signal = Signal::new(SIGHUP, handle).wait().expect("Can't setup signal listener");
    hup.map(|_| info!("Received SIGHUP."))
}

fn quit_signal(handle: &Handle) -> impl Stream
{
    let int: Signal  = Signal::new(SIGINT, handle).wait().expect("Can't setup signal listener");
    let term: Signal = Signal::new(SIGTERM, handle).wait().expect("Can't setup signal listener");

    let int  = int.map(|_| info!("Received SIGINT."));
    let term = term.map(|_| info!("Received SIGTERM."));

    int.merge(term)
}

fn tcp_listener(handle: &Handle) -> impl Future
{
    let reload_signal = reload_signal(handle).for_each(|_| {
        Ok(())
    }).map(|_| {}).map_err(|_| {});

    let addr = SocketAddr::from_str("0.0.0.0:6667").unwrap();
    let listener = TcpListener::bind(&addr, handle).expect("Can't bind tcp socket")
        .incoming()
        .for_each(|(socket, peer_addr)| {
            Ok(()) //handle.spawn(server());
        })
        .map(|_| {}).map_err(|_| {});

    reload_signal.select(listener)
}

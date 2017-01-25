extern crate futures;
extern crate tokio_core;
extern crate tokio_signal;
extern crate libc;

use self::futures::{Future, Stream};
use self::tokio_core::reactor::{Core, Handle};
use self::tokio_signal::unix::{Signal, SIGHUP, SIGINT, SIGTERM};
use self::libc::c_int;

pub fn run()
{
    let mut core = Core::new().expect("Can't create reactor, shouldn't happen!");
    let handle = core.handle();

    core.run(main(&handle));
}

fn main(handle: &Handle) -> impl Future
{
    let reload_signal = reload_signal(handle);
    let quit_signal   = quit_signal(handle);

    quit_signal.into_future()
}

fn reload_signal(handle: &Handle) -> impl Stream
{
    let hup: Signal = Signal::new(SIGHUP, handle).wait().unwrap();
    hup.map(|_| info!("Received SIGHUP."))
}

fn quit_signal(handle: &Handle) -> impl Stream
{
    let int: Signal  = Signal::new(SIGINT, handle).wait().unwrap();
    let term: Signal = Signal::new(SIGTERM, handle).wait().unwrap();

    let int  = int.map(|_| info!("Received SIGINT."));
    let term = term.map(|_| info!("Received SIGTERM."));

    int.merge(term)
}

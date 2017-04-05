#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate lircd;
extern crate bytes;
extern crate tokio_io;

use lircd::net::IrcCodec;
use bytes::BytesMut;
use tokio_io::codec::Decoder;

fuzz_target!(|data: &[u8]| {
    let mut buf = BytesMut::from(data);

    let mut decoder = IrcCodec;

    while buf.len() > 0 {
        let _ = decoder.decode(&mut buf);
    }
});

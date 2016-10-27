extern crate lircd;

use lircd::net;

fn main() {
    net::run(net::DEFAULT_LISTEN_ADDR);
}

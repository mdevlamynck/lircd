extern crate lircd;
extern crate env_logger;

use lircd::net;

fn main() {
    env_logger::init().unwrap_or_else(|e| {
        println!("ERROR: unable to init log");
        println!("ERROR: original error: {}", e);
    });

    net::run(net::DEFAULT_LISTEN_ADDR);
}

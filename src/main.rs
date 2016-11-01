extern crate lircd;
extern crate env_logger;

use lircd::net;
use lircd::config;
use lircd::irc::IrcProtocol;

fn main() {
    env_logger::init().unwrap_or_else(|e| {
        println!("ERROR: unable to init log");
        println!("ERROR: original error: {}", e);
    });

    let protocol = IrcProtocol::new();

    net::run(config::Config::default(), protocol);
}

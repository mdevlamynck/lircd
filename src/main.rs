extern crate lircd;
extern crate env_logger;

use lircd::net;
use lircd::config;

fn main() {
    env_logger::init().unwrap_or_else(|e| {
        println!("ERROR: unable to init log");
        println!("ERROR: original error: {}", e);
    });

    config::Config::create_if_doesnt_exist();
    net::run(config::Config::load());
}

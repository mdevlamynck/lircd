extern crate lircd;
extern crate env_logger;
#[macro_use]
extern crate docopt;

use std::path::PathBuf;
use docopt::Docopt;
use lircd::net;
use lircd::config;

const USAGE: &'static str = "Usage: lircd [options]

Options:
    -c, --config FILE  Path to the configuration file
    -h, --help         Print help and quit
    -v, --version      Print version information and quit
";

fn main() {
    let args = docopt!(USAGE);

    env_logger::init().unwrap_or_else(|e| {
        println!("ERROR: unable to init log");
        println!("ERROR: original error: {}", e);
    });

    let path = args.get_str("--config");
    let config = if !path.is_empty() {
        config::Config::load_from(PathBuf::from(path))
    } else {
        config::Config::load()
    };

    config.create_if_doesnt_exist();
    net::run(config);
}

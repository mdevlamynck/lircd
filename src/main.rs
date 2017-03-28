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

#![feature(plugin)]
#![plugin(indoc)]

extern crate lircd;
extern crate env_logger;
#[macro_use]
extern crate docopt;

use docopt::Docopt;
use lircd::config;
use lircd::net;

fn main() {
    env_logger::init().unwrap_or_else(|e| {
        println!("ERROR: unable to init log: {}", e);
    });

    let args = docopt!(indoc!("
        Usage: lircd [options]

        Options:
            -c, --config FILE  Path to the configuration file
            -h, --help         Print help and quit
            -v, --version      Print version information and quit
    "));

    config::create_or_load_from_path(args.get_str("--config"));

    net::run();
}

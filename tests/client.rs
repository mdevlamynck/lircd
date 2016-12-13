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

#![cfg(test)]

extern crate lircd;
extern crate rand;

mod functional
{
    use std::net::TcpStream;
    use std::thread;
    use std::time;
    use rand::{thread_rng, Rng};
    use std::str;

    use lircd::{net, config};

    const TEST_LISTEN_ADDR: &'static str = "127.0.0.1";

    fn init_serv() -> (&'static str, u16)
    {
        let port: u16                       = thread_rng().gen_range(6000, 6999);
        let mut config                      = config::Config::new();
        config.inner.network.listen_address = format!("{}:{}", TEST_LISTEN_ADDR, port);

        thread::spawn(move || {
            net::run(config);
        });

        thread::sleep(time::Duration::from_millis(1000));

        (TEST_LISTEN_ADDR, port)
    }

    #[test]
    fn can_connect()
    {
        let _ = TcpStream::connect(init_serv()).unwrap();
    }

    #[test]
    fn multiple_clients_can_connect()
    {
        let server = init_serv();
        let _ = TcpStream::connect(server).unwrap();
        let _ = TcpStream::connect(server).unwrap();
        let _ = TcpStream::connect(server).unwrap();
        let _ = TcpStream::connect(server).unwrap();
        let _ = TcpStream::connect(server).unwrap();
    }
}

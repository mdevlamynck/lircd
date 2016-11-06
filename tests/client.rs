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
        let port: u16         = thread_rng().gen_range(6000, 6999);
        let mut config        = config::Config::new();
        config.listen_address = format!("{}:{}", TEST_LISTEN_ADDR, port);

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

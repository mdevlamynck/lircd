#![cfg(test)]

extern crate lircd;
extern crate rand;

mod functional
{
    use std::io::{Write, BufRead, BufReader};
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

    #[test]
    fn round_echo_server()
    {
        let server = init_serv();
        let mut client1 = TcpStream::connect(server).unwrap();
        let mut client2 = TcpStream::connect(server).unwrap();

        let mut client1_reader = BufReader::new(client1.try_clone().unwrap());
        let mut client2_reader = BufReader::new(client2.try_clone().unwrap());

        let _ = client1.write(b"Hi!\n");
        let _ = client1.flush();

        let mut client1_buffer = String::new();
        let mut client2_buffer = String::new();

        let _ = client1_reader.read_line(&mut client1_buffer);
        let _ = client2_reader.read_line(&mut client2_buffer);

        assert_eq!("Hi!\n", &client1_buffer);
        assert_eq!("Hi!\n", &client2_buffer);

        let _ = client2.write(b"It works!\n");
        let _ = client2.flush();

        client1_buffer.clear();
        client2_buffer.clear();

        let _ = client1_reader.read_line(&mut client1_buffer);
        let _ = client2_reader.read_line(&mut client2_buffer);

        assert_eq!("It works!\n", &client1_buffer);
        assert_eq!("It works!\n", &client2_buffer);
    }
}

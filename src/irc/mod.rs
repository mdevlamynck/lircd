extern crate mioco;

use std::io::{self, Read, Write};
use std::sync::Arc;
use std::collections::HashMap;
use error::NetResult;
use self::mioco::sync::{Mutex, RwLock};
use net::{StatefullProtocol, StatefullHandle};
use reader::{MaxLengthedBufRead, MaxLengthedBufReader};
use config::Config;

mod message;
mod command;

type IrcState<Output> = Arc<RwLock<Irc<Output>>>;

pub struct IrcProtocol<Output>
    where Output: Write
{
    state: IrcState<Output>,
}

impl<Output> StatefullProtocol<Output> for IrcProtocol<Output>
    where Output: Write
{
    type Handle = IrcHandle<Output>;

    fn new(config: Config) -> Self
    {
        IrcProtocol::<Output> {
            state: Arc::new(RwLock::new(Irc::new(config))),
        }
    }

    fn new_connection(&self, output: Output) -> Self::Handle
    {
        IrcHandle::<Output>::new(self.state.clone(), output)
    }
}

pub struct IrcHandle<Output>
    where Output: Write
{
    state:      IrcState<Output>,
    connection: Mutex<Connection<Output>>,
}

impl<Output> StatefullHandle<Output> for IrcHandle<Output>
    where Output: Write
{
    fn consume<Input: Read>(self, input: Input) -> NetResult
    {
        let input_reader = MaxLengthedBufReader::new(input);

        for line in input_reader.lines_without_too_long() {
            let request = line.unwrap();

            try!(command::dispatch_command(&self, request));
        }

        Ok(())
    }
}

impl<Output> IrcHandle<Output>
    where Output: Write
{
    pub fn new(state_holder: IrcState<Output>, connection: Output) -> IrcHandle<Output>
    {
        IrcHandle::<Output> {
            state:      state_holder,
            connection: Mutex::new(Connection::Unknown(connection)),
        }
    }
}

pub struct Irc<Output>
{
    pub config:   RwLock<Config>,
    pub users:    RwLock<HashMap<String, Client<Output>>>,
    pub channels: RwLock<HashMap<String, Client<Output>>>,
}

impl<Output> Irc<Output>
{
    pub fn new(config: Config) -> Self
    {
        Irc::<Output> {
            config:   RwLock::new(config),
            users:    RwLock::new(HashMap::new()),
            channels: RwLock::new(HashMap::new()),
        }
    }
}

pub enum Connection<Output>
    where Output: Write
{
    Client(Client<Output>),
    Server(Server<Output>),
    Unknown(Output),
}

impl<Output> Write for Connection<Output>
    where Output: Write
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>
    {
        match *self {
            Connection::Client(ref mut client)  => unimplemented!(),
            Connection::Server(ref mut server)  => unimplemented!(),
            Connection::Unknown(ref mut output) => output.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()>
    {
        match *self {
            Connection::Client(ref mut client)  => unimplemented!(),
            Connection::Server(ref mut server)  => unimplemented!(),
            Connection::Unknown(ref mut output) => output.flush(),
        }
    }
}

pub struct Server<Output>
{
    pub output: Arc<Mutex<Output>>,

    pub name:   String,
}

pub struct Client<Output>
{
    pub output:      Arc<Mutex<Output>>,

    //pub nickname:    String, // user nickname
    //pub hostname:    String, // name of client's host
    //pub username:    String, // name of the user on that host
    //pub server:      String, // server the client is connected to
    //pub is_operator: bool,   // has operator rights on this server

    //pub channels:    Vec<Channel<Output>>,
}

pub struct Channel<Output>
{
    pub name:     String,

    pub users:    Vec<Client<Output>>,
    pub operator: String, // nickname of this channel's operator
}

impl<Output> Client<Output>
{
    pub fn new(output: Output) -> Self
    {
        Client::<Output> {
            output: Arc::new(Mutex::new(output)),
        }
    }

    pub fn clone(&self) -> Self
    {
        Client::<Output> {
            output: self.output.clone(),
        }
    }
}

#[cfg(test)]
mod test
{
    #[test]
    fn new_connection_adds_user_to_the_list_and_creates_the_handle()
    {
        // TODO
    }

    #[test]
    fn consume_reads_line_by_line_and_calls_handle_request()
    {
        // TODO
    }

    #[test]
    fn handle_request_writes_back_the_request_to_all_connected_users()
    {
        // TODO
    }
}

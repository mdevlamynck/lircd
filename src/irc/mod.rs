extern crate mioco;

use std::io::{Read, Write};
use std::sync::Arc;
use std::collections::HashMap;
use error::NetResult;
use self::mioco::sync::{Mutex, RwLock};
use net::{StatefullProtocol, StatefullHandle};
use reader::{MaxLengthedBufRead, MaxLengthedBufReader};
use config::Config;

mod message;

type IrcState<Output> = Arc<RwLock<Irc<Output>>>;

pub struct IrcProtocol<Output>
    where Output: Write + Send + 'static
{
    state: IrcState<Output>,
}

impl<Output> StatefullProtocol<Output> for IrcProtocol<Output>
    where Output: Write + Send + 'static
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
        IrcHandle::<Output>::new(&self.state, output)
    }
}

pub struct IrcHandle<Output>
    where Output: Write + Send + 'static
{
    state:      IrcState<Output>,
    connection: Connection<Output>,
}

impl<Output> StatefullHandle<Output> for IrcHandle<Output>
    where Output: Write + Send + 'static
{
    fn consume<Input: Read>(self, input: Input) -> NetResult
    {
        let input_reader = MaxLengthedBufReader::new(input);

        for line in input_reader.lines_without_too_long() {
            let request = line.unwrap();

            try!(self.handle_request(request));
        }

        Ok(())
    }
}

impl<Output> IrcHandle<Output>
    where Output: Write + Send + 'static
{
    pub fn new(state_holder: &IrcState<Output>, connection: Output) -> IrcHandle<Output>
    {
        IrcHandle::<Output> {
            state:      state_holder.clone(),
            connection: Connection::Unknown(connection),
        }
    }

    fn handle_request(&self, request: String) -> NetResult
    {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Irc<Output: 'static>
{
    pub config:   Config,
    pub users:    HashMap<String, Client<Output>>,
    pub channels: HashMap<String, Client<Output>>,
}

impl<Output> Irc<Output>
{
    pub fn new(config: Config) -> Self
    {
        Irc::<Output> {
            config:   config,
            users:    HashMap::new(),
            channels: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub enum Connection<Output>
    where Output: Write + Send + 'static
{
    Client(Client<Output>),
    Server(Server<Output>),
    Unknown(Output),
}

#[derive(Debug)]
pub struct Server<Output: 'static>
{
    pub output: Arc<Mutex<Output>>,

    pub name:   String,
}

#[derive(Debug)]
pub struct Client<Output: 'static>
{
    pub output:      Arc<Mutex<Output>>,

    //pub nickname:    String, // user nickname
    //pub hostname:    String, // name of client's host
    //pub username:    String, // name of the user on that host
    //pub server:      String, // server the client is connected to
    //pub is_operator: bool,   // has operator rights on this server

    //pub channels:    Vec<Channel<Output>>,
}

#[derive(Debug)]
pub struct Channel<Output: 'static>
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

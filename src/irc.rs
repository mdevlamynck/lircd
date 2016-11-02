extern crate mioco;

use std::io::{Read, Write};
use std::sync::Arc;
use self::mioco::sync::{Mutex, RwLock};
use errors::NetResult;
use net::{StatefullProtocol, StatefullHandle};
use reader::{MaxLengthedBufRead, MaxLengthedBufReader};

pub struct IrcProtocol<O: 'static>
{
    state: Arc<RwLock<Irc<O>>>,
}

impl<O> IrcProtocol<O>
{
    pub fn new() -> Self
    {
        IrcProtocol::<O> {
            state: Arc::new(RwLock::new(Irc::<O>::new())),
        }
    }

    pub fn clone(&self) -> Self
    {
        IrcProtocol::<O> {
            state: self.state.clone(),
        }
    }

    fn add_client(&self, client: Client<O>)
    {
        let mut state = self.state.write().unwrap();
        state.users.push(client);
    }
}

pub struct IrcHandle<O: 'static>
{
    protocol: IrcProtocol<O>,
    client:   Client<O>,
}

impl<O> IrcHandle<O>
    where O: Write
{
    pub fn new(state_holder: &IrcProtocol<O>, client: Client<O>) -> IrcHandle<O>
    {
        IrcHandle::<O> {
            protocol: state_holder.clone(),
            client:   client
        }
    }
}

impl<O> StatefullProtocol for IrcProtocol<O>
    where O: Write
{
    type O = O;
    type H = IrcHandle<O>;

    fn new_connection(&self, output: Self::O) -> Self::H
    {
        let client = Client::new(output);
        self.add_client(client.clone());

        IrcHandle::<O>::new(self, client)
    }
}

impl<O> StatefullHandle for IrcHandle<O>
    where O: Write
{
    fn consume<I: Read>(self, input: I) -> NetResult
    {
        let input_reader = MaxLengthedBufReader::new(input);

        for line in input_reader.lines_without_too_long() {
            let request = line.unwrap();

            try!(self.handle_request(request));
        }

        Ok(())
    }

    fn handle_request(&self, request: String) -> NetResult
    {
        let state = self.protocol.state.read().unwrap();

        for user in state.users.iter() {
            let mut output = user.output.lock().unwrap();
            try!(output.write(request.as_bytes()));
            try!(output.write(b"\n"));
            try!(output.flush());
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Irc<O: 'static>
{
    pub users: Vec<Client<O>>,
}

impl<O> Irc<O>
{
    pub fn new() -> Self
    {
        Irc::<O> {
            users: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Client<O: 'static>
{
    pub output: Arc<Mutex<O>>,
}

impl<O> Client<O>
{
    pub fn new(output: O) -> Self
    {
        Client::<O> {
            output: Arc::new(Mutex::new(output)),
        }
    }

    pub fn clone(&self) -> Self
    {
        Client::<O> {
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

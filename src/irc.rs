extern crate mioco;

use std::io::{Read, Write};
use std::sync::Arc;
use self::mioco::sync::{Mutex, RwLock};
use errors::NetResult;
use net::{StatefullProtocol, StatefullHandle};
use reader::{MaxLengthedBufRead, MaxLengthedBufReader};

pub struct IrcProtocol<Output>
    where Output: Write + Send + 'static
{
    state: Arc<RwLock<Irc<Output>>>,
}

impl<Output> IrcProtocol<Output>
    where Output: Write + Send + 'static
{
    pub fn clone(&self) -> Self
    {
        IrcProtocol::<Output> {
            state: self.state.clone(),
        }
    }

    fn add_client(&self, client: Client<Output>)
    {
        let mut state = self.state.write().unwrap();
        state.users.push(client);
    }
}

pub struct IrcHandle<Output>
    where Output: Write + Send + 'static
{
    protocol: IrcProtocol<Output>,
    client:   Client<Output>,
}

impl<Output> IrcHandle<Output>
    where Output: Write + Send + 'static
{
    pub fn new(state_holder: &IrcProtocol<Output>, client: Client<Output>) -> IrcHandle<Output>
    {
        IrcHandle::<Output> {
            protocol: state_holder.clone(),
            client:   client
        }
    }
}

impl<Output> StatefullProtocol<Output> for IrcProtocol<Output>
    where Output: Write + Send + 'static
{
    type Handle = IrcHandle<Output>;

    fn new() -> Self
    {
        IrcProtocol::<Output> {
            state: Arc::new(RwLock::new(Irc::<Output>::new())),
        }
    }

    fn new_connection(&self, output: Output) -> Self::Handle
    {
        let client = Client::new(output);
        self.add_client(client.clone());

        IrcHandle::<Output>::new(self, client)
    }
}

impl<Output> StatefullHandle<Output> for IrcHandle<Output>
    where Output: Write + Send + 'static
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
pub struct Irc<Output: 'static>
{
    pub users: Vec<Client<Output>>,
}

impl<Output> Irc<Output>
{
    pub fn new() -> Self
    {
        Irc::<Output> {
            users: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Client<Output: 'static>
{
    pub output: Arc<Mutex<Output>>,
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

extern crate mioco;

use std::io::{Read, Write};
use std::sync::Arc;
use self::mioco::sync::{Mutex, RwLock};
use errors::NetResult;
use net::{StatefullProtocol, StatefullHandle};
use reader::{MaxLengthedBufRead, MaxLengthedBufReader};
use std::marker::PhantomData;

pub struct IrcProtocol<I, O: 'static>
{
    state:      Arc<RwLock<Irc<O>>>,
    input_type: PhantomData<I>,
}

impl<I, O> IrcProtocol<I, O>
{
    pub fn new() -> Self
    {
        IrcProtocol::<I, O> {
            state:      Arc::new(RwLock::new(Irc::<O>::new())),
            input_type: PhantomData,
        }
    }

    pub fn clone(&self) -> Self
    {
        IrcProtocol::<I, O> {
            state:      self.state.clone(),
            input_type: PhantomData,
        }
    }
}

pub struct IrcHandle<I, O: 'static>
    where I: Read,
{
    input: MaxLengthedBufReader<I>,
    state: IrcProtocol<I, O>,
}

impl<I, O> IrcHandle<I, O>
    where I: Read,
          O: Write
{
    pub fn new(input: I, state_holder: &IrcProtocol<I, O>) -> IrcHandle<I, O>
    {
        IrcHandle::<I, O> {
            input: MaxLengthedBufReader::new(input),
            state: state_holder.clone(),
        }
    }
}

impl<I, O> StatefullProtocol for IrcProtocol<I, O>
    where I: Read,
          O: Write
{
    type I = I;
    type O = O;
    type H = IrcHandle<I, O>;

    fn new_connection(&self, input: Self::I, output: Self::O) -> Self::H
    {
        let mut state = self.state.write().unwrap();
        state.users.push(Client::new(output));

        IrcHandle::<I, O>::new(input, self)
    }

    fn handle_request(&self, request: String) -> NetResult
    {
        let state = self.state.read().unwrap();
        for user in state.users.iter() {
            let mut output = user.output.lock().unwrap();
            try!(output.write(request.as_bytes()));
            try!(output.write(b"\n"));
            try!(output.flush());
        }

        Ok(())
    }
}

impl<I, O> StatefullHandle for IrcHandle<I, O>
    where I: Read,
          O: Write
{
    fn consume(self) -> NetResult
    {
        for line in self.input.lines_without_too_long() {
            let request = line.unwrap();

            try!(self.state.handle_request(request));
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
    pub output: Mutex<O>,
}

impl<O> Client<O>
{
    pub fn new(output: O) -> Self
    {
        Client::<O> {
            output: Mutex::new(output),
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

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

extern crate resolve;

use std::io::{self, Read, Write};
use std::sync::Arc;
use std::collections::HashMap;
use error::{NetResult, NetError};
use mioco::sync::{Mutex, RwLock};
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

    fn new(mut config: Config) -> Self
    {
        if let Ok(hostname) = resolve::hostname::get_hostname() {
            config.network.hostname = hostname;
        }

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
    output:     Arc<Mutex<Output>>,
}

impl<Output> StatefullHandle<Output> for IrcHandle<Output>
    where Output: Write
{
    fn consume<Input: Read>(self, input: Input) -> NetResult
    {
        let input_reader = MaxLengthedBufReader::new(input);

        for line in input_reader.lines_without_too_long() {
            let request = line.unwrap();

            match command::dispatch_command(&self, request) {
                Ok(_)                          => continue,
                Err(NetError::CloseConnection) => break,
                Err(err)                       => return Err(err),
            }
        }

        Ok(())
    }
}

impl<Output> IrcHandle<Output>
    where Output: Write
{
    pub fn new(state_holder: IrcState<Output>, connection: Output) -> IrcHandle<Output>
    {
        let output = Arc::new(Mutex::new(connection));

        IrcHandle::<Output> {
            state:      state_holder,
            connection: Mutex::new(Connection::Unknown(output.clone())),
            output:     output,
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
    Unknown(Arc<Mutex<Output>>),
}

impl<Output> Write for Connection<Output>
    where Output: Write
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>
    {
        match *self {
            Connection::Client(ref mut client)  => client.write(buf),
            Connection::Server(ref mut server)  => server.write(buf),
            Connection::Unknown(ref mut output) => output.lock().unwrap().write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()>
    {
        match *self {
            Connection::Client(ref mut client)  => client.flush(),
            Connection::Server(ref mut server)  => server.flush(),
            Connection::Unknown(ref mut output) => output.lock().unwrap().flush(),
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

    pub nickname:    String, // user nickname
    //pub hostname:    String, // name of client's host
    pub username:    String, // name of the user on that host
    pub realname:    String, // name of the user on that host
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

impl<Output> Server<Output>
{
    pub fn new(output: Arc<Mutex<Output>>) -> Self
    {
        Server::<Output> {
            output: output,
            name:   String::new(),
        }
    }
}

impl<Output> Client<Output>
{
    pub fn new(output: Arc<Mutex<Output>>) -> Self
    {
        Client::<Output> {
            output:   output,
            nickname: String::new(),
            username: String::new(),
            realname: String::new(),
        }
    }
}

impl<Output> Write for Server<Output>
    where Output: Write
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>
    {
        self.output.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()>
    {
        self.output.lock().unwrap().flush()
    }
}

impl<Output> Write for Client<Output>
    where Output: Write
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>
    {
        self.output.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()>
    {
        self.output.lock().unwrap().flush()
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

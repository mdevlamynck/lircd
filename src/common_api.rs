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

extern crate mioco;

use std;
use std::io::{Read, Write};
use error::Result;
use std::thread;

pub trait Listen: Sized
{
    type Stream: Stream;

    fn bind(address: &str) -> Result<Self>;

    fn accept(&self) -> Result<Self::Stream>;
}

pub trait Stream: Read + Write + Sized + Send + 'static
{
    fn try_clone(&self) -> Result<Self>;
}

impl Listen for mioco::tcp::TcpListener
{
    type Stream = mioco::tcp::TcpStream;

    fn bind(address: &str) -> Result<Self>
    {
        let tcp_listen_address = address.parse()?;
        let tcp_listener       = Self::bind(&tcp_listen_address)?;

        Ok(tcp_listener)
    }

    fn accept(&self) -> Result<Self::Stream>
    {
        Ok(Self::accept(self)?)
    }
}

impl Stream for mioco::tcp::TcpStream
{
    fn try_clone(&self) -> Result<Self>
    {
        Ok(Self::try_clone(self)?)
    }
}

impl Listen for std::net::TcpListener
{
    type Stream = std::net::TcpStream;

    fn bind(address: &str) -> Result<Self>
    {
        let tcp_listener = Self::bind(&address)?;

        Ok(tcp_listener)
    }

    fn accept(&self) -> Result<Self::Stream>
    {
        let (stream, _) = Self::accept(self)?;

        Ok(stream)
    }
}

impl Stream for std::net::TcpStream
{
    fn try_clone(&self) -> Result<Self>
    {
        Ok(Self::try_clone(self)?)
    }
}

pub trait Spawn<T>
    where T: Send + 'static
{
    type JoinHandle: JoinHandle<T>;

    fn spawn<F>(f: F) -> Self::JoinHandle
        where F: FnOnce() -> T,
              F: Send + 'static;
}

pub struct Async;
pub struct Blocking;

impl<T> Spawn<T> for Async
    where T: Send + 'static
{
    type JoinHandle = mioco::JoinHandle<T>;

    fn spawn<F>(f: F) -> Self::JoinHandle
        where F: FnOnce() -> T,
              F: Send + 'static,
    {
        self::mioco::spawn(f)
    }
}

impl<T> Spawn<T> for Blocking
    where T: Send + 'static
{
    type JoinHandle = std::thread::JoinHandle<T>;

    fn spawn<F>(f: F) -> Self::JoinHandle
        where F: FnOnce() -> T,
              F: Send + 'static,
    {
        std::thread::spawn(f)
    }
}

pub trait JoinHandle<T>: Sized
    where T: Send + 'static
{
    fn join(self: Self) -> thread::Result<T>;
}

impl<T> JoinHandle<T> for mioco::JoinHandle<T>
      where T: Send + 'static
{
    fn join(self: Self) -> thread::Result<T>
    {
        Self::join(self)
    }
}

impl<T> JoinHandle<T> for thread::JoinHandle<T>
      where T: Send + 'static
{
    fn join(self: Self) -> thread::Result<T>
    {
        Self::join(self)
    }
}

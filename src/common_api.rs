extern crate mioco;

use std;
use std::io::{Read, Write};
use errors::Result;
use std::thread;
use std::sync::LockResult;
use std::sync::MutexGuard;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

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

pub trait Mutex<T>
    where T: Sized
{
    fn new(t: T) -> Self;

    fn lock(&self) -> LockResult<MutexGuard<T>>;
}

impl<T> Mutex<T> for mioco::sync::Mutex<T>
    where T: Sized
{
    fn new(t: T) -> Self
    {
        Self::new(t)
    }

    fn lock(&self) -> LockResult<MutexGuard<T>>
    {
        Self::lock(self)
    }
}

impl<T> Mutex<T> for std::sync::Mutex<T>
    where T: Sized
{
    fn new(t: T) -> Self
    {
        Self::new(t)
    }

    fn lock(&self) -> LockResult<MutexGuard<T>>
    {
        Self::lock(self)
    }
}

pub trait RwLock<T>
    where T: Sized
{
    fn new(t: T) -> Self;

    fn read(&self) -> LockResult<RwLockReadGuard<T>>;

    fn write(&self) -> LockResult<RwLockWriteGuard<T>>;
}

impl<T> RwLock<T> for mioco::sync::RwLock<T>
    where T: Sized
{
    fn new(t: T) -> Self
    {
        Self::new(t)
    }

    fn read(&self) -> LockResult<RwLockReadGuard<T>>
    {
        Self::read(self)
    }

    fn write(&self) -> LockResult<RwLockWriteGuard<T>>
    {
        Self::write(self)
    }
}

impl<T> RwLock<T> for std::sync::RwLock<T>
    where T: Sized
{
    fn new(t: T) -> Self
    {
        Self::new(t)
    }

    fn read(&self) -> LockResult<RwLockReadGuard<T>>
    {
        Self::read(self)
    }

    fn write(&self) -> LockResult<RwLockWriteGuard<T>>
    {
        Self::write(self)
    }
}

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
        let tcp_listen_address = try!(address.parse());
        let tcp_listener = try!(Self::bind(&tcp_listen_address));

        Ok(tcp_listener)
    }

    fn accept(&self) -> Result<Self::Stream>
    {
        Ok(try!(Self::accept(self)))
    }
}

impl Stream for mioco::tcp::TcpStream
{
    fn try_clone(&self) -> Result<Self>
    {
        Ok(try!(Self::try_clone(self)))
    }
}

impl Listen for std::net::TcpListener
{
    type Stream = std::net::TcpStream;

    fn bind(address: &str) -> Result<Self>
    {
        let tcp_listener = try!(Self::bind(&address));

        Ok(tcp_listener)
    }

    fn accept(&self) -> Result<Self::Stream>
    {
        let (stream, _) = try!(Self::accept(self));

        Ok(stream)
    }
}

impl Stream for std::net::TcpStream
{
    fn try_clone(&self) -> Result<Self>
    {
        Ok(try!(Self::try_clone(self)))
    }
}

impl Listen for mioco::unix::UnixListener
{
    type Stream = mioco::unix::UnixStream;

    fn bind(address: &str) -> Result<Self>
    {
        Ok(try!(Self::bind(address)))
    }

    fn accept(&self) -> Result<Self::Stream>
    {
        Ok(try!(Self::accept(self)))
    }
}

impl Stream for mioco::unix::UnixStream
{
    fn try_clone(&self) -> Result<Self>
    {
        Ok(try!(Self::try_clone(self)))
    }
}

impl Listen for std::os::unix::net::UnixListener
{
    type Stream = std::os::unix::net::UnixStream;

    fn bind(address: &str) -> Result<Self>
    {
        Ok(try!(Self::bind(address)))
    }

    fn accept(&self) -> Result<Self::Stream>
    {
        let (stream, _) = try!(Self::accept(self));

        Ok(stream)
    }
}

impl Stream for std::os::unix::net::UnixStream
{
    fn try_clone(&self) -> Result<Self>
    {
        Ok(try!(Self::try_clone(self)))
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
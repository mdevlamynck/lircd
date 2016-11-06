extern crate mioco;

use std::{net, io, result, error, fmt};

#[derive(Debug)]
pub enum NetError
{
    Io(io::Error),
    Parse(net::AddrParseError),
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NetError::Io(ref err)    => write!(f, "IO error: {}", err),
            NetError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for NetError
{
    fn description(&self) -> &str
    {
        match *self {
            NetError::Io(ref err)    => err.description(),
            NetError::Parse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error>
    {
        match *self {
            NetError::Io(ref err)    => Some(err),
            NetError::Parse(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for NetError
{
    fn from(err: io::Error) -> NetError
    {
        NetError::Io(err)
    }
}

impl From<net::AddrParseError> for NetError
{
    fn from(err: net::AddrParseError) -> NetError
    {
        NetError::Parse(err)
    }
}

pub type NetResult = result::Result<(), NetError>;
pub type Result<T> = result::Result<T, NetError>;

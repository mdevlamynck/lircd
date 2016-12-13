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

use std::{net, io, result, fmt};
use std::error::Error;

#[derive(Debug)]
pub enum NetError
{
    Io(io::Error),
    Parse(net::AddrParseError),
    CloseConnection,
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NetError::Io(ref err)     => write!(f, "IO error: {}", err),
            NetError::Parse(ref err)  => write!(f, "Parse error: {}", err),
            NetError::CloseConnection => write!(f, "{}", self.description()),
        }
    }
}

impl Error for NetError
{
    fn description(&self) -> &str
    {
        match *self {
            NetError::Io(ref err)     => err.description(),
            NetError::Parse(ref err)  => err.description(),
            NetError::CloseConnection => "Close connection",
        }
    }

    fn cause(&self) -> Option<&Error>
    {
        match *self {
            NetError::Io(ref err)     => Some(err),
            NetError::Parse(ref err)  => Some(err),
            NetError::CloseConnection => None,
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

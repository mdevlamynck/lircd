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

#![feature(conservative_impl_trait)]

#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate mioco;
extern crate simple_signal;
extern crate unindent;
#[macro_use]
extern crate lazy_static;

pub mod irc;
pub mod net;
pub mod config;
pub mod error;
pub mod reader;

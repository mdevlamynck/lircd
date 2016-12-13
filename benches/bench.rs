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

#![cfg_attr(feature = "unstable", feature(test))]
#![cfg(all(feature = "unstable", test))]

extern crate test;
extern crate lircd;

use std::io::{BufRead, BufReader};
use lircd::reader::{MaxLengthedBufRead, MaxLengthedBufReader};

#[bench]
fn reference_buf_read_lines_only_line_returns(b: &mut test::Bencher)
{
    let buffer = ['\n' as u8; 1024];

    b.iter(move || {
        let buf_reader = BufReader::new(buffer.as_ref());

        for _ in buf_reader.lines() {}
    });
}

#[bench]
fn max_lengthed_buf_read_lines_without_too_long_only_line_returns(b: &mut test::Bencher)
{
    let buffer = ['\n' as u8; 1024];

    b.iter(move || {
        let buf_reader = MaxLengthedBufReader::new(buffer.as_ref());

        for _ in buf_reader.lines_without_too_long() {}
    });
}

#[bench]
fn reference_buf_read_lines_no_line_returns(b: &mut test::Bencher)
{
    let buffer = [' ' as u8; 1024];

    b.iter(move || {
        let buf_reader = BufReader::new(buffer.as_ref());

        for _ in buf_reader.lines() {}
    });
}

#[bench]
fn max_lengthed_buf_read_lines_without_too_long_no_line_returns(b: &mut test::Bencher)
{
    let buffer = [' ' as u8; 1024];

    b.iter(move || {
        let buf_reader = MaxLengthedBufReader::new(buffer.as_ref());

        for _ in buf_reader.lines_without_too_long() {}
    });
}

#[bench]
fn reference_buf_read_lines_some_line_returns(b: &mut test::Bencher)
{
    let mut buffer = [' ' as u8; 1024];
    for i in 1..11 {
        buffer[1024 / i - 1] = '\n' as u8;
    }

    b.iter(move || {
        let buf_reader = BufReader::new(buffer.as_ref());

        for _ in buf_reader.lines() {}
    });
}

#[bench]
fn max_lengthed_buf_read_lines_without_too_long_some_line_returns(b: &mut test::Bencher)
{
    let mut buffer = [' ' as u8; 1024];
    for i in 1..11 {
        buffer[1024 / i - 1] = '\n' as u8;
    }


    b.iter(move || {
        let buf_reader = MaxLengthedBufReader::new(buffer.as_ref());

        for _ in buf_reader.lines_without_too_long() {}
    });
}

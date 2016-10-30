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

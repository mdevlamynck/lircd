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

extern crate memchr;

use std::io::{self, Read, BufRead, BufReader};
use std::io::ErrorKind;
use std::str;

/// IRC max line lenght * max utf8 char size
pub const MAX_BUFFER_SIZE: usize = 512 * 4;

/// Adds the notion of a max length in the lines read by a BufRead.
pub trait MaxLengthedBufRead : Read
{
    /// This function acts like read_until from BufRead. The only difference is that the
    /// function will return ErrorKind::Interrupted in case of lines too long. The content of the
    /// fautive line is discarded until a new line.
    fn read_until_char_or_max(&mut self, byte: u8, buf: &mut Vec<u8>) -> io::Result<usize>;

    /// This function acts like lines from BufRead. The only difference is that the function will
    /// skip over lines wich are longer than self.max_length.
    fn lines_without_too_long(self) -> Lines<Self> where Self: Sized
    {
        Lines { buf: self }
    }
}

pub struct MaxLengthedBufReader<R: Read>
{
    buf:        BufReader<R>,
    max_length: usize,
}

impl<R: Read> MaxLengthedBufReader<R>
{
    pub fn new(r: R) -> MaxLengthedBufReader<R>
    {
        MaxLengthedBufReader::<R> {
            buf:        BufReader::new(r),
            max_length: MAX_BUFFER_SIZE,
        }
    }

    pub fn set_max_line(&mut self, max_length: usize)
    {
        self.max_length = max_length;
    }
}

impl<R: Read> Read for MaxLengthedBufReader<R>
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
    {
        self.buf.read(buf)
    }
}

impl<R: Read> MaxLengthedBufRead for MaxLengthedBufReader<R>
{
    fn read_until_char_or_max(&mut self, byte: u8, buf: &mut Vec<u8>) -> io::Result<usize>
    {
        let mut read = 0;
        loop {
            let (found, used) = {
                let available = match self.buf.fill_buf() {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e)
                };
                match memchr::memchr(byte, available) {
                    Some(i) => {
                        buf.extend_from_slice(&available[..i + 1]);
                        (true, i + 1)
                    }
                    None => {
                        buf.extend_from_slice(available);
                        (false, available.len())
                    }
                }
            };
            self.buf.consume(used);
            read += used;

            if read > self.max_length {
                return Err(io::Error::new(ErrorKind::Interrupted, "Buffer reached max length"));
            }

            if found || used == 0 {
                return Ok(read);
            }
        }
    }
}

pub struct Lines<R: MaxLengthedBufRead>
{
    buf: R,
}

impl<R: MaxLengthedBufRead> Iterator for Lines<R>
{
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>>
    {
        let mut next_line = String::new();
        let mut skip_line = false;
        let mut read_result;
        loop {
            read_result = append_to_string(&mut next_line, |b| self.buf.read_until_char_or_max('\n' as u8, b));
            match read_result {
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {
                    next_line.clear();
                    skip_line = true;
                },
                _ => {
                    if skip_line {
                        skip_line = false;
                        next_line.clear();
                        continue;
                    }
                    break;
                }
            }
        }

        match read_result {
            Ok(0) => None,
            Ok(_n) => {
                if next_line.ends_with("\n") {
                    next_line.pop();
                    if next_line.ends_with("\r") {
                        next_line.pop();
                    }
                }
                Some(Ok(next_line))
            },
            Err(e) => Some(Err(e))
        }
    }
}

fn append_to_string<F>(buf: &mut String, f: F) -> io::Result<usize>
    where F: FnOnce(&mut Vec<u8>) -> io::Result<usize>
{
    struct Guard<'a> { s: &'a mut Vec<u8>, len: usize }
        impl<'a> Drop for Guard<'a> {
        fn drop(&mut self) {
            unsafe { self.s.set_len(self.len); }
        }
    }

    unsafe {
        let mut g = Guard { len: buf.len(), s: buf.as_mut_vec() };
        let ret = f(g.s);
        if str::from_utf8(&g.s[g.len..]).is_err() {
            ret.and_then(|_| {
                Err(io::Error::new(ErrorKind::InvalidData,
                               "stream did not contain valid UTF-8"))
            })
        } else {
            g.len = g.s.len();
            ret
        }
    }
}

#[cfg(test)]
mod test
{
    use std::io::ErrorKind;
    use super::{MaxLengthedBufRead, MaxLengthedBufReader, MAX_BUFFER_SIZE};

    #[test]
    fn returns_error_interrupt_on_line_too_long()
    {
        let input_buf      = [0u8; MAX_BUFFER_SIZE + 1];
        let mut output_buf = vec![0u8; 0];
        let mut buf_reader = MaxLengthedBufReader::new(input_buf.as_ref());

        let result = buf_reader.read_until_char_or_max(0xA, &mut output_buf);
        assert!(result.is_err());
        assert_eq!(ErrorKind::Interrupted, result.err().unwrap().kind());
    }

    #[test]
    fn max_length_is_updatable()
    {
        const SHORTED_MAX_LINE: usize = 10;
        let input_buf                 = [0u8; SHORTED_MAX_LINE + 1];
        let mut output_buf            = vec![0u8; 0];
        let mut buf_reader            = MaxLengthedBufReader::new(input_buf.as_ref());

        buf_reader.set_max_line(SHORTED_MAX_LINE);
        assert_eq!(SHORTED_MAX_LINE, buf_reader.max_length);

        let result = buf_reader.read_until_char_or_max(0xA, &mut output_buf);
        assert!(result.is_err());
        assert_eq!(ErrorKind::Interrupted, result.err().unwrap().kind());
    }

    #[test]
    fn reads_until_end_of_buffer()
    {
        let input_buf      = [0u8; MAX_BUFFER_SIZE];
        let mut output_buf = vec![0u8; 0];
        let mut buf_reader = MaxLengthedBufReader::new(input_buf.as_ref());

        let result = buf_reader.read_until_char_or_max(0xA, &mut output_buf);
        assert!(result.is_ok());
    }

    #[test]
    fn reads_until_char()
    {
        let mut input_buf  = [0u8; MAX_BUFFER_SIZE + 100];
        input_buf[9]       = '\n' as u8;
        input_buf[19]      = '\n' as u8;
        let mut output_buf = vec![0u8; 0];
        let mut buf_reader = MaxLengthedBufReader::new(input_buf.as_ref());

        let result = buf_reader.read_until_char_or_max('\n' as u8, &mut output_buf);
        assert!(result.is_ok());
        assert_eq!(10, output_buf.len());
    }

    #[test]
    fn reads_until_char_or_max_or_end()
    {
        let mut input_buf               = [0u8; MAX_BUFFER_SIZE + 100];
        input_buf[9]                    = '\n' as u8;
        input_buf[19]                   = '\n' as u8;
        input_buf[MAX_BUFFER_SIZE + 29] = '\n' as u8;
        input_buf[MAX_BUFFER_SIZE + 39] = '\n' as u8;
        let mut output_buf              = vec![0u8; 0];
        let mut buf_reader              = MaxLengthedBufReader::new(input_buf.as_ref());

        // 1st \n
        let result = buf_reader.read_until_char_or_max('\n' as u8, &mut output_buf);
        assert!(result.is_ok());
        assert_eq!(10, output_buf.len());

        output_buf.clear();

        // 2nd \n
        let result = buf_reader.read_until_char_or_max('\n' as u8, &mut output_buf);
        assert!(result.is_ok());
        assert_eq!(10, output_buf.len());

        output_buf.clear();

        // Too long line \n
        let result = buf_reader.read_until_char_or_max(0xA, &mut output_buf);
        assert!(result.is_err());
        assert_eq!(ErrorKind::Interrupted, result.err().unwrap().kind());

        output_buf.clear();

        // 3rd \n: content between too long data and the 3rd \n
        let result = buf_reader.read_until_char_or_max('\n' as u8, &mut output_buf);
        assert!(result.is_ok());
        assert_eq!(10, output_buf.len());

        output_buf.clear();

        // 4th \n
        let result = buf_reader.read_until_char_or_max('\n' as u8, &mut output_buf);
        assert!(result.is_ok());
        assert_eq!(60, output_buf.len());

        output_buf.clear();

        // Buffer reached the end on previous read
        let result = buf_reader.read_until_char_or_max(0xA, &mut output_buf);
        assert!(result.is_ok());
        assert_eq!(0, result.unwrap());
    }

    #[test]
    fn discards_too_long_lines()
    {
        let mut input_buf               = [0u8; MAX_BUFFER_SIZE + 100];
        input_buf[9]                    = '\n' as u8;
        input_buf[19]                   = '\n' as u8;
        input_buf[MAX_BUFFER_SIZE + 29] = '\n' as u8;
        input_buf[MAX_BUFFER_SIZE + 39] = '\n' as u8;
        let buf_reader                  = MaxLengthedBufReader::new(input_buf.as_ref());
        let mut iterator                = buf_reader.lines_without_too_long();

        // 1st \n
        let result = iterator.next();
        assert!(result.is_some());
        assert_eq!(9, result.unwrap().unwrap().len());

        // 2nd \n
        let result = iterator.next();
        assert!(result.is_some());
        assert_eq!(9, result.unwrap().unwrap().len());

        // Too long line discarded
        // Content until 3rd \n discarded too
        // 4th \n
        let result = iterator.next();
        assert!(result.is_some());
        assert_eq!(60, result.unwrap().unwrap().len());

        // Buffer reached the end on previous read
        let result = iterator.next();
        assert!(result.is_none());
    }
}

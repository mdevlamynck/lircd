extern crate memchr;

use std::io::{self, Read, BufRead, BufReader};
use std::io::ErrorKind;

/// IRC max line lenght * max utf8 char size
pub const MAX_BUFFER_SIZE: usize = 512 * 4;

/// Adds the notion of a max length in the lines read by a BufRead.
pub trait MaxLengthedBufRead : Read
{
    /// This function acts like read_until from BufRead. The only difference is that the
    /// function will return ErrorKind::Interrupted in case of lines too long. The content of the
    /// fautive line is discarded until a new line.
    fn read_until_char_or_max(&mut self, byte: u8, buf: &mut Vec<u8>) -> io::Result<usize>;

    fn lines_without_too_long(self) -> Lines<Self> where Self: Sized
    {
        Lines { buf: self }
    }
}

pub struct MaxLengthedBufReader<R: Read>
{
    buf: BufReader<R>,
}

impl<R: Read> MaxLengthedBufReader<R>
{
    pub fn new(r: R) -> MaxLengthedBufReader<R>
    {
        MaxLengthedBufReader::<R> {
            buf: BufReader::new(r),
        }
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

            if read > MAX_BUFFER_SIZE {
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
        let mut buf = Vec::<u8>::new();
        let mut next_line;
        let mut skip_line = false;
        let mut read_result;
        loop {
            read_result = self.buf.read_until_char_or_max('\n' as u8, &mut buf);
            match read_result {
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {
                    buf.clear();
                    skip_line = true;
                },
                _ => {
                    if skip_line {
                        skip_line = false;
                        buf.clear();
                        continue;
                    }
                    next_line = String::from_utf8_lossy(&buf).into_owned();
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

#[cfg(test)]
mod test
{
    extern crate memchr;

    use std::io::ErrorKind;
    use super::{MaxLengthedBufRead, MaxLengthedBufReader};

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

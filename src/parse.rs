use std::{fmt::Debug, io::Read};

const BUF_SIZE: usize = 1 << 25;
const MAX_DATA_LEN: usize = 1 << 25;

#[derive(Clone)]
pub enum Command {
    Get(Vec<u8>),
    Set(Vec<u8>, Vec<u8>),
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get(key) => {
                let k = std::str::from_utf8(key).unwrap_or("[Invalid ASCII]");
                write!(f, "Get {k}")
            }
            Self::Set(key, value) => {
                let k = std::str::from_utf8(key).unwrap_or("[Invalid ASCII]");
                let v = std::str::from_utf8(value).unwrap_or("[Invalid ASCII]");
                write!(f, "Set {k} {v}")
            }
        }
    }
}

pub struct CommandStream<R: Read> {
    reader: R,
    buffer: Vec<u8>,
    start_idx: usize,
}

enum ParseError {
    InvalidPrefix,
    NeedMoreData,
}

impl<R: Read> CommandStream<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::with_capacity(BUF_SIZE),
            start_idx: 0,
        }
    }

    fn parse_command(&mut self) -> Result<Command, ParseError> {
        self.start_idx = 0;

        if self.buffer.len() < 3 {
            return Err(ParseError::NeedMoreData);
        }
        let operation = &self.buffer[0..3];
        let ret;
        if operation == [b'G', b'E', b'T'] {
            self.start_idx = 3;
            ret = self.parse_get()?;
        } else if operation == [b'S', b'E', b'T'] {
            self.start_idx = 3;
            ret = self.parse_set()?;
        } else {
            return Err(ParseError::InvalidPrefix);
        }

        // make sure newline present at the end of the command
        if *self
            .buffer
            .get(self.start_idx)
            .ok_or(ParseError::NeedMoreData)?
            != b'\n'
        {
            return Err(ParseError::InvalidPrefix);
        }

        // drain buffer upon total matching success; + 1 because of newline
        self.buffer.drain(0..self.start_idx + 1);

        Ok(ret)
    }

    fn parse_get(&mut self) -> Result<Command, ParseError> {
        let key = self.parse_string()?;

        Ok(Command::Get(key))
    }

    fn parse_set(&mut self) -> Result<Command, ParseError> {
        let key = self.parse_string()?;
        let value = self.parse_string()?;

        Ok(Command::Set(key, value))
    }

    fn parse_string(&mut self) -> Result<Vec<u8>, ParseError> {
        // Note: per the spec <int> ::= [1-9] ([0-9])*
        let mut iter = self.buffer.iter().skip(self.start_idx);

        let mut data_length: usize = 0;
        let mut prefix_length: usize = 2;
        if let Some(&fst) = iter.next() {
            if fst == b'$' {
                for &next in iter {
                    if next == b'$' {
                        break;
                    }
                    let num = (next as char)
                        .to_digit(10)
                        .ok_or(ParseError::InvalidPrefix)?;
                    if data_length == 0 && num == 0 {
                        return Err(ParseError::InvalidPrefix);
                    }
                    data_length = data_length * 10 + (num as usize);
                    prefix_length += 1;
                }
            }
        }

        if data_length == 0 || data_length > MAX_DATA_LEN {
            return Err(ParseError::InvalidPrefix);
        }

        if self.start_idx + prefix_length + data_length > self.buffer.len() {
            return Err(ParseError::NeedMoreData);
        }

        let data: Vec<u8> = self.buffer
            [self.start_idx + prefix_length..self.start_idx + prefix_length + data_length]
            .to_vec();
        self.start_idx += prefix_length + data_length;

        Ok(data)
    }
}

impl<R: Read> Iterator for CommandStream<R> {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        match self.parse_command() {
            Ok(command) => return Some(command),
            Err(ParseError::InvalidPrefix) => return None,
            _ => (),
        }
        let mut read_buf = [0; 4096];
        loop {
            match self.reader.read(&mut read_buf) {
                Ok(0) => {
                    // End of stream
                    if self.buffer.is_empty() {
                        return None;
                    }

                    // Try to parse what's left in the buffer
                    return self.parse_command().ok();
                }
                Ok(n) => {
                    self.buffer.extend_from_slice(&read_buf[..n]);
                    match self.parse_command() {
                        Ok(command) => return Some(command),
                        Err(ParseError::InvalidPrefix) => return None,
                        _ => (),
                    }
                }
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }
        }
    }
}

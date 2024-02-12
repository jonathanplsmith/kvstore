use std::fmt::Debug;

pub enum Command<'a> {
    Get(&'a [u8]),
    Set(&'a [u8], &'a [u8]),
}

impl<'a> Debug for Command<'a> {
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

pub fn parse_command(command: &[u8]) -> Option<Command> {
    if command.len() >= 3 {
        let operation = &command[0..3];
        if operation == [b'G', b'E', b'T'] {
            return parse_get(&command[3..]);
        } else if operation == [b'S', b'E', b'T'] {
            return parse_set(&command[3..]);
        }
    }
    None
}

// TODO: potentially change this to Box<[u8]>?
pub fn binary_safe_encode(string: &[u8]) -> Vec<u8> {
    let mut ret = Vec::from(string);
    let length = string.len().to_string();
    let len_bytes = length.bytes();
    ret.insert(0, b'$');
    ret.splice(0..0, len_bytes);
    ret.insert(0, b'$');

    ret
}

fn parse_get(command: &[u8]) -> Option<Command> {
    let (key, _) = parse_string(command)?;
    // TODO: potentially assert command fully used

    Some(Command::Get(key))
}

fn parse_set(command: &[u8]) -> Option<Command> {
    let (key, next) = parse_string(command)?;
    let (value, _) = parse_string(next)?;

    // TODO: potentially assert command fully used
    Some(Command::Set(key, value))
}

fn parse_string(command: &[u8]) -> Option<(&[u8], &[u8])> {
    // Note: per the spec <int> ::= [1-9] ([0-9])*
    let mut iter = command.iter();

    let mut length: usize = 0;
    let mut int_digits: usize = 2;
    if let Some(&fst) = iter.next() {
        if fst == b'$' {
            for &next in iter {
                if next == b'$' {
                    break;
                }
                let num = (next as char).to_digit(10)?;
                length = length * 10 + (num as usize);
                int_digits += 1;
            }
        }
    }

    if length == 0 {
        return None; // must be None, as the first digit needs to be at least 1
    }

    if int_digits + length <= command.len() {
        let string = &command[int_digits..int_digits + length];
        let next = &command[int_digits + length..];
        Some((string, next))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::binary_safe_encode;

    #[test]
    fn encode_1() {
        let hello = "Hello, World!";

        let enc = binary_safe_encode(hello.as_bytes());

        assert_eq!("$13$Hello, World!".as_bytes(), enc);
    }
}

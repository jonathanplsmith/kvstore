use crate::store::Reponse;

pub fn encode_response(resp: Reponse) -> Vec<u8> {
    match resp {
        Reponse::Ok => encode_ok(),
        Reponse::Err => encode_err(),
        Reponse::Value(data) => encode_value(data.as_ref()),
    }
}

fn encode_value(value: &[u8]) -> Vec<u8> {
    let mut value_v = binary_safe_encode(value);
    let mut ret = Vec::from("VALUE".as_bytes());
    ret.append(&mut value_v);
    ret.push(b'\n');

    ret
}

fn encode_ok() -> Vec<u8> {
    vec![b'O', b'K', b'\n']
}

fn encode_err() -> Vec<u8> {
    vec![b'E', b'R', b'R', b'\n']
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

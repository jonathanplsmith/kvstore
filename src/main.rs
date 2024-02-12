use std::{
    error::Error,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use kvstore::{encode, store::KVStore};

const ADDRESS: &str = "127.0.0.1:5555";
const BUFSIZE: usize = 1 << 25;

fn main() -> Result<(), Box<dyn Error>> {
    let listener =
        TcpListener::bind(ADDRESS).unwrap_or_else(|_| panic!("Error listening on {ADDRESS}"));

    let mut store: KVStore = KVStore::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap(); // Okay, as iterator never returns None

        // TODO: thread pool stuff
        handle_connection(stream, &mut store)?;
    }

    Ok(())
}

fn handle_connection(stream: TcpStream, store: &mut KVStore) -> io::Result<()> {
    let mut stream_write = stream.try_clone()?;
    let buffer = BufReader::with_capacity(BUFSIZE, stream);

    for command_res in buffer.lines() {
        match command_res {
            Ok(command) => {
                let bytes = command.as_bytes();
                if let Some(resp) = store.exec_command(bytes) {
                    let encoded = encode::encode_response(resp);
                    stream_write.write_all(&encoded)?
                } else {
                    stream_write.shutdown(std::net::Shutdown::Both)?;
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

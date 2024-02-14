use std::{
    error::Error,
    io::{self, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

use kvstore::{encode, parse::CommandStream, store::KVStore};

const ADDRESS: &str = "127.0.0.1:5555";

fn main() -> Result<(), Box<dyn Error>> {
    let listener =
        TcpListener::bind(ADDRESS).unwrap_or_else(|_| panic!("Error listening on {ADDRESS}"));

    let mut store: KVStore = KVStore::new();

    for stream in listener.incoming() {
        let stream = stream?;

        // TODO: thread pool stuff
        handle_connection(stream, &mut store)?;
    }

    Ok(())
}

fn handle_connection(stream: TcpStream, store: &mut KVStore) -> io::Result<()> {
    std::thread::sleep(std::time::Duration::from_millis(10));

    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let commands = CommandStream::new(reader);

    for command in commands {
        let res = store.exec_command(command);
        let encoded = encode::encode_response(res);
        writer.write_all(&encoded)?;
    }

    writer.flush()?;

    Ok(())
}

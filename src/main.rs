use std::{
    error::Error,
    io::Read,
    net::{TcpListener, TcpStream},
};

use kvstore::store::KVStore;

const ADDRESS: &str = "127.0.0.1:5555";

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

fn handle_connection(mut stream: TcpStream, store: &mut KVStore) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    loop {
        let size = stream.read(&mut buffer)?;
        if size == 0 {
            eprintln!("Connection closed");
            break;
        }

        let commands: Vec<&[u8]> = buffer[0..size].split(|&b| b == b'\n').collect();

        for command in commands {
            // TODO: error handling/response
            store.exec_command(command);
        }
    }

    Ok(())
}

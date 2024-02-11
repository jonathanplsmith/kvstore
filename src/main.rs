use std::{
    clone,
    error::Error,
    io::Read,
    net::{TcpListener, TcpStream},
};

use kvstore::protocol;
use kvstore::state::RuntimeState;

const ADDRESS: &str = "127.0.0.1:5555";

fn main() -> Result<(), Box<dyn Error>> {
    let listener =
        TcpListener::bind(ADDRESS).unwrap_or_else(|_| panic!("Error listening on {ADDRESS}"));

    let state: RuntimeState = RuntimeState::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap(); // Okay, as iterator never returns None

        // TODO: thread pool stuff
        handle_connection(stream, &state)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, state: &RuntimeState) -> Result<(), Box<dyn Error>> {
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
            protocol::exec_command(command);
        }
    }

    Ok(())
}

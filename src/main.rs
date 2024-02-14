use kvstore::{encode, parse::CommandStream, store::KVStore};
use std::{
    error::Error,
    io::{self, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};
use threadpool::ThreadPool;

const ADDRESS: &str = "127.0.0.1:5555";
const NUM_THREADS: usize = 32;

fn main() -> Result<(), Box<dyn Error>> {
    let listener =
        TcpListener::bind(ADDRESS).unwrap_or_else(|_| panic!("Error listening on {ADDRESS}"));

    let pool = ThreadPool::new(NUM_THREADS);
    let store = Arc::new(Mutex::new(KVStore::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        let store = store.clone();

        pool.execute(move || handle_connection(stream, store).unwrap());
    }

    Ok(())
}

fn handle_connection(stream: TcpStream, store: Arc<Mutex<KVStore>>) -> io::Result<()> {
    std::thread::sleep(std::time::Duration::from_millis(10));

    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let commands = CommandStream::new(reader);

    for command in commands {
        let res = store.lock().unwrap().exec_command(command);
        let encoded = encode::encode_response(res);
        writer.write_all(&encoded)?;
    }

    writer.flush()?;

    Ok(())
}

use clap::Parser;
use kvstore::{encode, parse::CommandStream, store::KVStore};
use std::{
    error::Error,
    io::{self, BufReader, BufWriter, Write},
    net::{Ipv4Addr, TcpListener, TcpStream, ToSocketAddrs},
    sync::{Arc, Mutex},
};
use threadpool::ThreadPool;

const DEFAULT_NUM_THREADS: usize = 32;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Config {
    #[arg(default_value_t = Ipv4Addr::new(127, 0, 0, 1))]
    address: Ipv4Addr,

    #[arg(default_value_t = 5555)]
    port: u16,

    #[arg(short, long)]
    threads: Option<usize>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::parse();

    let socket_addr = (config.address, config.port)
        .to_socket_addrs()?
        .next()
        .unwrap(); // okay, as parser makes sure values in range
    let num_threads = config.threads.unwrap_or(DEFAULT_NUM_THREADS);
    let listener = TcpListener::bind(socket_addr)
        .unwrap_or_else(|e| panic!("Error listening on {socket_addr}: {e}"));

    let pool = ThreadPool::new(num_threads);
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

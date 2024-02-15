# kvstore

A simple in-memory key-value store that can be communicated with via TCP. The primary purpose of this project was to get familiar with Rust, so I do not recommend using this for anything important.

Installation
--
Either clone or download the repository, then run `cargo build --release`. You will find the `kvstore` binary in the `target/release` directory.

Usage
--
To start the server, simply execute `./kvstore`, assuming the binary is in your current working directory. The server will start listening on `localhost:5555` by default. 

The IPv4 address/port may also be given as `./kvstore [IPv4] [Port]`. The `--threads <Threads>` option controls the number of threads in the thread pool answering requets. The default here is 32.

`./kvstore --help` also displays this information.

Protocol
--
The server will listen for the following commands:

- **Get**: retrieves the value associated with the provided key.
- **Set**: associates the given key with the given value.
- **Delete**: removes the given key from the store.
- **Clear**: removes all key-value pairs from the store.

The exact grammar for the commands is as follows:
```
command = get | set | delete | clear
get = "GET" data "\n"
set = "SET" data data "\n"
delete = "DEL" data "\n"
clear = "CLR\n"
data = "$" length "$" char+
length = [1-9] [0-9]*
char = <any byte>
```
where length corresponds to the lenth of the data and is at least 1 and at most $2^{25}$.

If the requests succeeds, the server will respond `"OK\n"` for Set, Delete, and Clear; for Get it will repond `"VALUE" data "\n"`. 
If the requests cannot be executed, eg. trying to get or delete a non-existent key, the server will repond `"ERR\n"`.
If a request does not adhere to the above grammar, the connection will be closed. 

Credit
--
The core of the protocol was taken from the bonus project of HS2022's iteration of Computer Systems at ETH Zurich.
